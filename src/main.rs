mod plugins;

fn main() {
    let mut plugins = plugins::Plugins::new("./plugins");

    print!("{plugins}");

    plugins.plugins.iter_mut().for_each(|plugin| {
        let plugin: &mut dyn common::plugin::Plugin = plugin.get_plugin_mut();
        println!("{}:", plugin.name());
        println!("{}", plugin.status());
    });

    plugins.plugins.iter().for_each(|plugin| {
        plugin.destroy();
    });
}
