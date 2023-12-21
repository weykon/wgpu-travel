use crate::atom::config::Config;
use crate::atom::ready::Ready;
use crate::handle::Handler;

pub struct App {
    pub states: Handler<dyn Ready>,
    pub configs: Handler<dyn Config>,
}

impl App {
    pub(crate) fn ready(&self) {
        self.states.process(self);
    }
}
