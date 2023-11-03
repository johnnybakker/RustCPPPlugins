
use rust_plugin::Plugin;

#[macro_use]
extern crate rust_plugin;

#[derive(Debug, Default)]
struct MyPlugin;

impl MyPlugin {
	fn new() -> Self {
		println!("Creating MyPlugin");
		Self
	}
}

impl Plugin for MyPlugin {
    fn name(&self) -> &'static str {
        "MyPlugin"
    }
}

declare_plugin!(MyPlugin, MyPlugin::new);