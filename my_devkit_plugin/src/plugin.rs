use std::any::Any;

pub trait Plugin : Any + Sync + Send {
    fn name(&self) -> &'static str;
    fn on_plugin_load(&mut self) {}
    fn on_plugin_unload(&mut self) {}
}