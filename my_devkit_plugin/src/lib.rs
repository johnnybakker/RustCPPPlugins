mod plugin;

//#[cfg(create_type = "cdylib")]
pub mod ffi;
pub use my_devkit_plugin_macros::declare_plugin;
pub use plugin::Plugin;