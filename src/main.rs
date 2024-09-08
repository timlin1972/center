use std::io::Write;

use anstream::println;
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use crossbeam_channel::unbounded;
use owo_colors::colors::xterm::Gray;
use owo_colors::OwoColorize as _;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::signal;
use tokio::time::{self, sleep, Duration};
use tracing::{info, span, Level};

use common::{cfg, utils};

mod plugins;

const MODULE: &str = "center";
const POLLING_TIMEOUT: u64 = 300;
const EXIT_INTER: &str = "exit@inter";
const EXIT_POLLING: &str = "exit@polling";

fn prompt() -> Result<(), String> {
    let datetime_local: DateTime<Local> = DateTime::from_timestamp(utils::get_ts() as i64, 0)
        .unwrap()
        .with_timezone(&Local);

    print!(
        "{} {}",
        datetime_local.format("%H:%M:%S").fg::<Gray>(),
        "> ".blue().bold()
    );
    std::io::stdout().flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("Panic occurred: {:?}", info);
        std::process::exit(1); // 立即退出程序
    }));

    // log
    tracing_subscriber::fmt::init();
    let span = span!(Level::INFO, MODULE);
    let _enter = span.enter();

    // cfg
    cfg::init();
    println!("Welcome to {}", cfg::get_name().blue());

    // plugins
    let mut plugins = plugins::Plugins::new();

    // channels
    let (tx_bridge, mut rx_bridge) = tokio::sync::mpsc::channel(100);
    let (tx_polling, mut rx_polling) = tokio::sync::mpsc::channel(100);
    let (tx_inter, rx_inter) = unbounded::<String>();

    // inter to bridge
    let tx_bridge_clone = tx_bridge.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(msg) = rx_inter.try_recv() {
                // info!("[rx] {msg}");
                if msg == EXIT_INTER {
                    break;
                }
                let _ = tx_bridge_clone.send(msg).await;
            }
            sleep(Duration::from_millis(50)).await;
        }
    });

    // startup and polling
    let tx_bridge_clone = tx_bridge.clone();
    tokio::spawn(async move {
        println!("[{}] Running the startup cmds.", MODULE.blue());
        for cmd in cfg::get_startup() {
            println!("[{}] {}", MODULE.blue(), cmd);
            tx_bridge_clone.send(cmd).await.unwrap();
        }

        async fn polling(tx_bridge_clone: &tokio::sync::mpsc::Sender<String>) {
            for cmd in cfg::get_polling() {
                // println!("[{}] {}", MODULE.blue(), cmd);
                tx_bridge_clone.send(cmd).await.unwrap();
            }
        }

        println!("[{}] Start the polling cmds.", MODULE.blue());
        loop {
            polling(&tx_bridge_clone).await;
            match time::timeout(Duration::from_secs(POLLING_TIMEOUT), rx_polling.recv()).await {
                Ok(Some(msg)) => {
                    if msg == EXIT_POLLING {
                        break;
                    }
                }
                Ok(None) => {
                    break;
                }
                Err(_) => (),
            }
        }
    });

    // stdin
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    prompt()?;
    let mut last_line = String::new();
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("Ctrl-C pressed, exiting...");
                tx_inter.send("exit".to_owned()).unwrap();
            },
            Some(line) = rx_bridge.recv() => {
                // info!("[rx_bridge] {line}");
                match respond(&line, &mut plugins
                    , &tx_inter
                ).await {
                    Ok(quit) => {
                        if quit {
                            tx_inter.send(EXIT_INTER.to_owned()).unwrap();
                            tx_polling.send(EXIT_POLLING.to_owned()).await.unwrap();
                                            std::process::exit(1);
                        }
                    }
                    Err(err) => println!("{err}"),
                }
            },
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // EOF reached
                    Ok(_) => {
                        let line = line.trim();
                        if line.is_empty() {
                            prompt()?;
                            continue;
                        }

                        let mut alias_cmd = None;
                        let aliases = cfg::get_aliases();
                        for alias in &aliases {
                            if alias.alias == line {
                                alias_cmd = Some(alias.cmd.clone());
                            }
                        }

                        let line = if alias_cmd.is_some() {
                            alias_cmd.unwrap()
                        } else if line == "!!" {
                                last_line
                            }
                            else {
                                line.to_owned()
                        };

                        match respond(&line, &mut plugins
                            , &tx_inter
                        ).await {
                            Ok(quit) => {
                                if quit {
                                    tx_inter.send(EXIT_INTER.to_owned()).unwrap();
                                    tx_polling.send(EXIT_POLLING.to_owned()).await.unwrap();
                                    break;
                                }
                            }
                            Err(err) => println!("{err}"),
                        }
                        last_line = line;
                        prompt()?;

                        let log = utils::encrypt(&format!("{}: {}", "User input".green(), last_line.replace('"', "\\\"")));
                        tx_inter.send(format!(r#"send plugin logs add '{log}'"#)).unwrap();
                    }
                    Err(e) => eprintln!("讀取標準輸入時發生錯誤: {}", e),
                }
                line.clear();
            },
            else => break,
        }
    }

    println!("{}", "Good bye.".blue());

    Ok(())
}

async fn respond(
    line: &str,
    plugins: &mut plugins::Plugins,
    tx: &crossbeam_channel::Sender<String>,
) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;

    match cli.command {
        Commands::Restart { subcommand } => match subcommand {
            RestartCommands::Plugins => {
                plugins.unload();
                plugins.load("./plugins", tx);
            }
            RestartCommands::Plugin { name } => {
                plugins.unload_plugin(&name);
                plugins.load_plugin("./plugins", tx, &name);
            }
        },
        Commands::Exit => {
            println!("{}", "Exiting...".blue());
            plugins.unload();
            return Ok(true);
        }
        Commands::Load { subcommand } => match subcommand {
            LoadCommands::Plugins => {
                plugins.load("./plugins", tx);
            }
            LoadCommands::Plugin { name } => {
                plugins.load_plugin("./plugins", tx, &name);
            }
        },
        Commands::Unload { subcommand } => match subcommand {
            UnloadCommands::Plugins => {
                plugins.unload();
            }
            UnloadCommands::Plugin { name } => {
                plugins.unload_plugin(&name);
            }
        },
        Commands::Show { subcommand } => match subcommand {
            ShowCommands::Plugins => {
                plugins.show();
            }
            ShowCommands::Plugin { name } => match plugins.get_plugin_mut(&name) {
                Ok(plugin) => {
                    plugin.show();
                }
                Err(e) => {
                    println!("Failed to show. {}", e.red());
                }
            },
        },
        Commands::Status { subcommand } => match subcommand {
            StatusCommands::Plugins => {
                plugins.status();
            }
            StatusCommands::Plugin { name } => match plugins.get_plugin_mut(&name) {
                Ok(plugin) => {
                    plugin.status();
                }
                Err(e) => {
                    println!("Failed to status. {}", e.red());
                }
            },
        },
        Commands::Send { subcommand } => match subcommand {
            SendCommands::Plugin { name, action, data } => match plugins.get_plugin_mut(&name) {
                Ok(plugin) => {
                    plugin.send(&action, &data);
                }
                Err(e) => {
                    println!("Failed to send. {}", e.red());
                }
            },
        },
        Commands::Action { subcommand } => match subcommand {
            ActionCommands::Plugin {
                name,
                action,
                data,
                data2,
            } => match plugins.get_plugin_mut(&name) {
                Ok(plugin) => {
                    plugin.action(&action, &data, &data2);
                }
                Err(e) => {
                    println!("Failed to action. {}", e.red());
                }
            },
        },
    }

    Ok(false)
}

#[derive(Debug, Parser)]
#[command(
    name = "my_app",
    // about = "A brief description of the application",
    after_help = r#"
Useful commands:
load plugins
    Load plugins
unload plugins
    Unload plugins
show plugins
    Show plugins
status plugins
    Show all plugins' status
status plugin {plugin}
    Show status of plugin
    plugin: mqtt / sysinfo / logs / devinfo /cfg
send plugin devinfo refresh all
    Request all devices to refresh the status
    "#
)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Restart {
        #[command(subcommand)]
        subcommand: RestartCommands,
    },
    Load {
        #[command(subcommand)]
        subcommand: LoadCommands,
    },
    Unload {
        #[command(subcommand)]
        subcommand: UnloadCommands,
    },
    Show {
        #[command(subcommand)]
        subcommand: ShowCommands,
    },
    Status {
        #[command(subcommand)]
        subcommand: StatusCommands,
    },
    Action {
        #[command(subcommand)]
        subcommand: ActionCommands,
    },
    Send {
        #[command(subcommand)]
        subcommand: SendCommands,
    },
    Exit,
}

#[derive(Debug, Subcommand)]
enum RestartCommands {
    Plugins,
    Plugin { name: String },
}

#[derive(Debug, Subcommand)]
enum LoadCommands {
    Plugins,
    Plugin { name: String },
}

#[derive(Debug, Subcommand)]
enum UnloadCommands {
    Plugins,
    Plugin { name: String },
}

#[derive(Debug, Subcommand)]
enum ShowCommands {
    Plugins,
    Plugin { name: String },
}

#[derive(Debug, Subcommand)]
enum StatusCommands {
    Plugins,
    Plugin { name: String },
}

#[derive(Debug, Subcommand)]
enum ActionCommands {
    Plugin {
        name: String,
        action: String,
        data: String,
        #[arg(default_value_t = String::from(""))]
        data2: String,
    },
}

#[derive(Debug, Subcommand)]
enum SendCommands {
    Plugin {
        name: String,
        action: String,
        data: String,
    },
}
