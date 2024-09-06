use std::fmt;
use std::fs;

use libloading::{Library, Symbol};

use common::plugin;

pub struct Plugin {
    path: String,
    lib: Library,
    plugin_wrapper: *mut plugin::PluginWrapper,
}

impl Plugin {
    pub fn new(path: String) -> Self {
        let (lib, plugin_wrapper) = unsafe {
            let lib = Library::new(&path).unwrap();
            let create_plugin: Symbol<unsafe extern "C" fn() -> *mut plugin::PluginWrapper> =
                lib.get(b"create_plugin").unwrap();
            let plugin_wrapper = create_plugin();
            (lib, plugin_wrapper)
        };

        println!("Load: {}", &path);

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

    pub fn destroy(&self) {
        let plugin = self.get_plugin();
        println!("Destory: {}", plugin.name());
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
        writeln!(f, "path: {}", self.path)?;
        writeln!(f, "name: {}", plugin.name())?;
        Ok(())
    }
}

pub struct Plugins {
    pub plugins: Vec<Plugin>,
}

impl Plugins {
    pub fn new(path: &str) -> Self {
        let mut plugins: Vec<Plugin> = vec![];

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "so" {
                            plugins.push(Plugin::new(path.display().to_string()));
                        }
                    }
                }
            }
        }

        Self { plugins }
    }
}

impl fmt::Display for Plugins {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.plugins.iter().for_each(|plugin| {
            writeln!(f, "plugin:").unwrap();
            writeln!(f, "{plugin}").unwrap();
        });
        Ok(())
    }
}
