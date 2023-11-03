use rust_plugin::{ Plugin, declare_plugin };

struct MyPlugin(i32);

impl Plugin for MyPlugin {
    fn name(&self) -> &'static str {
        "MyPlugin"
    }

	fn on_plugin_load(&mut self) {
		self.0 += 1;
		println!("Loading my plugin {}", self.0)
	}

	fn on_plugin_unload(&mut self) {
		println!("Unload my plugin")
	}
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self(Default::default())
    }
}


declare_plugin!(MyPlugin, MyPlugin::default);