use std::fmt;
use std::fs;

use libloading::{Library, Symbol};

use common::plugin;

const MODULE: &str = "plugins";

const PLUGINS_NOT_LOADED: &str = "Plugins not loaded. Please 'load plugins' first.";

pub struct Plugin {
    path: String,
    lib: Library,
    plugin_wrapper: *mut plugin::PluginWrapper,
}

impl Plugin {
    pub fn new(path: String) -> Self {
        println!("[{}] Loading: {}", MODULE, &path);

        let (lib, plugin_wrapper) = unsafe {
            let lib = Library::new(&path).unwrap();
            let create_plugin: Symbol<unsafe extern "C" fn() -> *mut plugin::PluginWrapper> =
                lib.get(b"create_plugin").unwrap();
            let plugin_wrapper = create_plugin();
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

    pub fn send(&mut self, data: &serde_json::Value) {
        let plugin = self.get_plugin_mut();
        println!("Send: {}", plugin.name());
        plugin.send(data);
    }

    pub fn destroy(&mut self) {
        let plugin: &mut dyn common::plugin::Plugin = self.get_plugin_mut();
        plugin.destroy();
        unsafe {
            let destroy_plugin: Symbol<unsafe extern "C" fn(*mut plugin::PluginWrapper)> =
                self.lib.get(b"destroy_plugin").unwrap();
            destroy_plugin(self.plugin_wrapper);
        }
    }
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let plugin = self.get_plugin();
        writeln!(f, "[{}]", plugin.name())?;
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

    pub fn load(&mut self, path: &str) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "so" {
                            self.plugins.push(Plugin::new(path.display().to_string()));
                        }
                    }
                }
            }
        }
    }

    pub fn send(&mut self, data: &serde_json::Value) {
        if self.plugins.is_empty() {
            println!("{}", PLUGINS_NOT_LOADED);
            return;
        }

        self.plugins.iter_mut().for_each(|plugin| {
            plugin.send(data);
        });
    }

    pub fn destroy(&mut self) {
        if self.plugins.is_empty() {
            println!("{}", PLUGINS_NOT_LOADED);
            return;
        }

        while let Some(mut plugin) = self.plugins.pop() {
            plugin.destroy();
        }
    }

    pub fn show(&self) {
        if self.plugins.is_empty() {
            println!("{}", PLUGINS_NOT_LOADED);
            return;
        }

        self.plugins.iter().for_each(|plugin| {
            println!("{plugin}");
        });
    }

    pub fn status(&mut self) {
        if self.plugins.is_empty() {
            println!("{}", PLUGINS_NOT_LOADED);
            return;
        }

        self.plugins.iter_mut().for_each(|plugin| {
            let plugin: &mut dyn common::plugin::Plugin = plugin.get_plugin_mut();
            println!("[{}]", plugin.name());
            println!("{}", plugin.status());
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
