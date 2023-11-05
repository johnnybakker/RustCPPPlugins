use my_devkit_plugin::{Plugin, declare_plugin};

#[derive(Default)]
#[declare_plugin(name = "my_plugin")]
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


#[derive(Default)]
#[declare_plugin(name = "my_second_plugin")]
struct MySecondPlugin(i32);

impl Plugin for MySecondPlugin {
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