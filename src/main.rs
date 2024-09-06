use std::io::Write;

use clap::{Parser, Subcommand};
use tokio::io::{self, AsyncBufReadExt, BufReader};
// use tokio::sync::mpsc;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

mod plugins;

fn prompt() -> Result<(), String> {
    print!("$ ");
    std::io::stdout().flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // let (tx, mut rx) = mpsc::channel(100); // 建立 mpsc 通道
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = LinesStream::new(reader.lines());
    prompt()?;

    // tokio::spawn(async move {
    //     // 模擬發送訊息到通道
    //     for i in 0..10 {
    //         tx.send(format!("訊息 {}", i)).await.unwrap();
    //         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    //     }
    // });

    let mut plugins = plugins::Plugins::new();

    loop {
        tokio::select! {
            Some(line) = lines.next() => {
                let line = line.unwrap();
                let line = line.trim();
                if line.is_empty() {
                    prompt()?;
                    continue;
                }
                match respond(line, &mut plugins) {
                    Ok(quit) => {
                        if quit {
                            break;
                        }
                    }
                    Err(err) => println!("{err}"),
                }
                prompt()?;
            },
            // Some(msg) = rx.recv() => {
            //     println!("從通道接收到: {}", msg);
            // },
            else => break,
        }
    }

    Ok(())
}

fn respond(line: &str, plugins: &mut plugins::Plugins) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;

    match cli.command {
        Commands::Exit => {
            println!("Exiting...");
            plugins.destroy();
            return Ok(true);
        }
        Commands::Show { subcommand } => match subcommand {
            ShowCommands::Plugins => {
                plugins.show();
            }
            ShowCommands::Plugin { name } => match name.as_str() {
                "sysinfo" => {
                    println!("sysinfo");
                }
                "temperature" => {
                    println!("temperature");
                }
                _ => println!("Unknown plugin: {name}"),
            },
        },
        Commands::Status { subcommand } => match subcommand {
            StatusCommands::Plugins => {
                plugins.status();
            }
            StatusCommands::Plugin { name } => match name.as_str() {
                "sysinfo" => {
                    println!("sysinfo");
                }
                "temperature" => {
                    println!("temperature");
                }
                _ => println!("Unknown plugin: {name}"),
            },
        },
        Commands::Load { subcommand } => match subcommand {
            LoadCommands::Plugins => {
                plugins.load("./plugins");
            }
            LoadCommands::Plugin { name } => match name.as_str() {
                "sysinfo" => {
                    println!("sysinfo");
                }
                "temperature" => {
                    println!("temperature");
                }
                _ => println!("Unknown plugin: {name}"),
            },
        },
        Commands::Send { subcommand } => match subcommand {
            SendCommands::Plugins => {
                plugins.send(&serde_json::json!("test"));
            }
            SendCommands::Plugin { name } => match plugins.get_plugin_mut(&name) {
                Ok(plugin) => {
                    plugin.send(&serde_json::json!("test"));
                }
                Err(e) => {
                    println!("{e}");
                }
            },
        },
        Commands::Destroy { subcommand } => match subcommand {
            DestroyCommands::Plugins => {
                plugins.destroy();
            }
            DestroyCommands::Plugin { name } => match name.as_str() {
                "sysinfo" => {
                    println!("sysinfo");
                }
                "temperature" => {
                    println!("temperature");
                }
                _ => println!("Unknown plugin: {name}"),
            },
        },
    }

    Ok(false)
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Show {
        #[command(subcommand)]
        subcommand: ShowCommands,
    },
    Status {
        #[command(subcommand)]
        subcommand: StatusCommands,
    },
    Load {
        #[command(subcommand)]
        subcommand: LoadCommands,
    },
    Send {
        #[command(subcommand)]
        subcommand: SendCommands,
    },
    Destroy {
        #[command(subcommand)]
        subcommand: DestroyCommands,
    },
    Exit,
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
enum LoadCommands {
    Plugins,
    Plugin { name: String },
}

#[derive(Debug, Subcommand)]
enum SendCommands {
    Plugins,
    Plugin { name: String },
}

#[derive(Debug, Subcommand)]
enum DestroyCommands {
    Plugins,
    Plugin { name: String },
}
