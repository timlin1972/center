use std::fmt;
use std::fs;

use anstream::println;
use libloading::{Library, Symbol};
use owo_colors::OwoColorize as _;
use tracing::{span, Level};

use common::plugin;

const MODULE: &str = "plugins";

const PLUGINS_NOT_LOADED: &str = "Plugins not loaded. Please 'load plugins' first.";

pub struct Plugin {
    path: String,
    lib: Library,
    plugin_wrapper: *mut plugin::PluginWrapper,
}

impl Plugin {
    pub fn new(path: String, tx: &crossbeam_channel::Sender<String>) -> Self {
        let span = span!(Level::INFO, MODULE);
        let _enter = span.enter();

        println!("[{}] Loading: {path}.", MODULE.blue());

        let (lib, plugin_wrapper) = unsafe {
            let lib = Library::new(&path).unwrap();
            let create_plugin: Symbol<
                unsafe extern "C" fn(
                    &crossbeam_channel::Sender<String>,
                ) -> *mut plugin::PluginWrapper,
            > = lib.get(b"create_plugin").unwrap();
            let plugin_wrapper = create_plugin(tx);
            (lib, plugin_wrapper)
        };

        Plugin {
            path,
            lib,
            plugin_wrapper,
        }
    }

    pub fn get_plugin(&self) -> &dyn plugin::Plugin {
        unsafe { &mut *self.plugin_wrapper }.plugin.as_ref()
    }

    pub fn get_plugin_mut(&mut self) -> &mut dyn plugin::Plugin {
        unsafe { &mut *self.plugin_wrapper }.plugin.as_mut()
    }

    pub fn unload(&mut self) {
        let plugin: &mut dyn common::plugin::Plugin = self.get_plugin_mut();
        plugin.unload();
        unsafe {
            let unload_plugin: Symbol<unsafe extern "C" fn(*mut plugin::PluginWrapper)> =
                self.lib.get(b"unload_plugin").unwrap();
            unload_plugin(self.plugin_wrapper);
        }
    }
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let plugin = self.get_plugin();
        writeln!(f, "[{}]", plugin.name().blue())?;
        writeln!(f, "\tpath: {}", self.path)?;
        writeln!(f, "\tname: {}", plugin.name())?;
        Ok(())
    }
}

pub struct Plugins {
    plugins: Vec<Plugin>,
}

impl Plugins {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn load(&mut self, path: &str, tx: &crossbeam_channel::Sender<String>) {
        if !self.plugins.is_empty() {
            println!("Failed to load. {}", "Plugins loaded.".red());
            return;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "so" || extension == "dylib" {
                            self.plugins
                                .push(Plugin::new(path.display().to_string(), tx));
                        }
                    }
                }
            }
        }
    }

    pub fn load_plugin(&mut self, path: &str, tx: &crossbeam_channel::Sender<String>, name: &str) {
        if self.get_plugin_mut(name).is_ok() {
            println!(
                "Failed to load_plugin. {}",
                format!("Plugin {name} is existed.").red()
            );
            return;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_stem() {
                        let name = format!("libtln_{}", name);
                        if name == filename.to_str().unwrap() {
                            if let Some(extension) = path.extension() {
                                if extension == "so" || extension == "dylib" {
                                    self.plugins
                                        .push(Plugin::new(path.display().to_string(), tx));
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(
            "Failed to load_plugin. {}",
            format!("Plugin {name} not found.").red()
        );
    }

    pub fn unload(&mut self) {
        if self.plugins.is_empty() {
            println!("Failed to unload. {}", PLUGINS_NOT_LOADED.red());
            return;
        }

        while let Some(mut plugin) = self.plugins.pop() {
            plugin.unload();
        }
    }

    pub fn unload_plugin(&mut self, name: &str) {
        if self.plugins.is_empty() {
            println!("Failed to unload_plugin. {}", PLUGINS_NOT_LOADED.red());
            return;
        }

        if let Some(pos) = self
            .plugins
            .iter()
            .position(|plugin| plugin.get_plugin().name() == name)
        {
            self.plugins[pos].unload();
            self.plugins.remove(pos);
            return;
        }

        println!(
            "Failed to unload_plugin. {}",
            format!("Plugin {name} not found.").red()
        );
    }

    pub fn show(&self) {
        if self.plugins.is_empty() {
            println!("Failed to show. {}", PLUGINS_NOT_LOADED.red());
            return;
        }

        self.plugins.iter().for_each(|plugin| {
            println!("{plugin}");
        });
    }

    pub fn status(&mut self) {
        if self.plugins.is_empty() {
            println!("Failed to status. {}", PLUGINS_NOT_LOADED.red());
            return;
        }

        self.plugins.iter_mut().for_each(|plugin| {
            let plugin: &mut dyn common::plugin::Plugin = plugin.get_plugin_mut();
            plugin.status();
        });
    }

    pub fn get_plugin_mut(&mut self, name: &str) -> Result<&mut dyn plugin::Plugin, String> {
        if self.plugins.is_empty() {
            return Err(PLUGINS_NOT_LOADED.to_owned());
        }

        for plugin in &mut self.plugins {
            let plugin: &mut dyn common::plugin::Plugin = plugin.get_plugin_mut();
            if plugin.name() == name {
                return Ok(plugin);
            }
        }

        Err(format!("Err: Plugin '{name}' not found"))
    }
}

impl fmt::Display for Plugins {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for plugin in &self.plugins {
            writeln!(f, "plugin:")?;
            writeln!(f, "{plugin}")?;
        }

        Ok(())
    }
}
