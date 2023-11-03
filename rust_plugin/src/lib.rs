
pub trait Plugin {
    fn name(&self) -> &'static str;
    fn on_plugin_load(&mut self) {}
    fn on_plugin_unload(&mut self) {}
}

pub type PluginCreate = unsafe fn() -> PluginWrapper;
pub const PLUGIN_CREATE_SYMBOL: &[u8] = b"_plugin_create";

#[repr(C)]
pub struct PluginWrapper {
	pub ptr: *mut (),
	pub on_load: extern fn(*mut()),
	pub on_unload: extern fn(*mut())
}

impl PluginWrapper {
	
}

#[derive(Debug, Default)]
struct TestPlugin;

impl Plugin for TestPlugin {
    fn name(&self) -> &'static str {
        "Test"
    }
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub unsafe extern "C" fn _plugin_create() -> $crate::PluginWrapper {
			let constructor: fn() -> $plugin_type = $constructor;
			let object = constructor();
			let boxed: Box<$plugin_type> = Box::new(object);
			let ptr = Box::into_raw(boxed);
	
			let on_load = <$plugin_type>::on_plugin_load 
				as fn(&mut $plugin_type)
				as *const fn(*mut());
	
			let on_unload = <$plugin_type>::on_plugin_unload 
				as fn(&mut $plugin_type)
				as *const fn(*mut());
	
			let wrap = $crate::PluginWrapper {
				ptr: std::mem::transmute(ptr),
				on_load: std::mem::transmute(on_load),
				on_unload: std::mem::transmute(on_unload)
			};
		
			wrap
        }
    };
}