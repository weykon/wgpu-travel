use crate::config::Config;
use crate::ready::Ready;

pub struct App {
    pub states: Handler<dyn Ready>,
    pub configs: Handler<dyn Config>,
}