
pub trait Plugin {
    fn name(&self) -> &'static str;
    fn on_plugin_load(&self) {}
    fn on_plugin_unload(&self) {}
}

pub type PluginCreate = unsafe fn() -> *mut ();
pub const PLUGIN_CREATE_SYMBOL: &[u8] = b"_plugin_create";

#[repr(C)]
pub struct PluginWrapper {
	pub ptr: *mut (),
	pub on_load: fn(*mut()),
	pub on_unload: fn(*mut())
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

unsafe fn test() {

	let constructor: fn() -> TestPlugin = TestPlugin::default;
	let object = constructor();
	let boxed: Box<dyn Plugin> = Box::new(object);
	let ptr = Box::into_raw(boxed);

	let on_load = TestPlugin::on_plugin_load as fn(&TestPlugin);
	on_load as fn(*const TestPlugin);

	let wrap = PluginWrapper {
		ptr,
		on_load = fn() {},
		on_unload: TestPlugin::on_plugin_load as fn(*mut())
	};
	

}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut () {
            let constructor: fn() -> $plugin_type = $constructor;
            let object = constructor();
            let boxed: Box<dyn $crate::Plugin> = Box::new(object);
            let ptr = Box::into_raw(boxed) as *mut ()
	
        }
    };
}