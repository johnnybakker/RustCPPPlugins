#[repr(C)]
pub struct Plugin {
	pub ptr: *mut (),
	pub on_load: extern fn(*mut()),
	pub on_unload: extern fn(*mut()),
}

impl Plugin {

	pub unsafe extern "C" fn on_plugin_load(&mut self) {
		(self.on_load)(self.ptr);
	}

	pub unsafe extern "C" fn on_plugin_unload(&mut self) {
		(self.on_unload)(self.ptr);
	}
	
}

impl<P: crate::Plugin> From<P> for Plugin {
    fn from(value: P) -> Self {
		let boxed = Box::new(value);
		let ptr = Box::into_raw(boxed);
	
		let on_load = 
			P::on_plugin_load as fn(&mut P) as *const fn(*mut());
		
		let on_unload = 
			P::on_plugin_unload as fn(&mut P) as *const fn(*mut());
	
		unsafe {
			Self {
				ptr: std::mem::transmute(ptr),
				on_load: std::mem::transmute(on_load),
				on_unload: std::mem::transmute(on_unload)
			}
		}
    }
}