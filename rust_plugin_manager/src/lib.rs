extern crate libloading;
extern crate rust_plugin;

use std::{
	path::Path, 
	io::Error, fmt::Formatter, any::Any
};

use libloading::{Library, Symbol};
use rust_plugin::{Plugin, PluginCreate, PLUGIN_CREATE_SYMBOL};


#[no_mangle]
pub unsafe extern "C" fn print_hello() {
    println!("Hello World!");
	
	let exe_path = std::env::current_exe().unwrap();
	let plugins_dir = exe_path.parent().unwrap().join("plugins");
	let mut manager = PluginManager::new();
	let _ = manager.load_plugins(plugins_dir);
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

	pub unsafe fn load_plugins<T: AsRef<Path>>(&mut self, path: T) -> Result<(), Error>{
	
		println!("Load plugins from {:?}",path.as_ref());

		for entry in std::fs::read_dir(path)? {
			if let Ok(item) = entry {
				let path = item.path();
				let _ = self.load_plugin(path);
			}
		}
		
		Ok(())
	}	

    pub unsafe fn load_plugin<P: AsRef<Path>>(&mut self, path: P) {
		println!("Load plugin from {:?}",path.as_ref());
        let lib = Library::new(path.as_ref()).unwrap();
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

		println!("Finding symbol {:?}",PLUGIN_CREATE_SYMBOL);
        let constructor: Symbol<PluginCreate> = lib.get(PLUGIN_CREATE_SYMBOL).unwrap();

		println!("Found symbol {:?}",PLUGIN_CREATE_SYMBOL);
        let wrapper = constructor();
		println!("Got plugin");
		(wrapper.on_load)(wrapper.ptr);
		(wrapper.on_load)(wrapper.ptr);
		(wrapper.on_load)(wrapper.ptr);
		(wrapper.on_load)(wrapper.ptr);
		println!("Finding symbol {:?}",PLUGIN_CREATE_SYMBOL);
        //let plugin = Box::from_raw(boxed_raw);
        //println!("Loaded plugin: {}", plugin.name());
        //plugin.on_plugin_load();
        //self.plugins.push(plugin);
    }

    pub fn unload(&mut self) {
        println!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            println!("Firing on_plugin_unload for {:?}", plugin.name());
            //plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}

impl core::fmt::Debug for PluginManager {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let plugins: Vec<_> = self.plugins.iter().map(|p| p.name()).collect();

        f.debug_struct("PluginManager")
            .field("plugins", &plugins)
            .finish()
    }
}

pub fn my_rust_fun() {
	println!("WTF");
}