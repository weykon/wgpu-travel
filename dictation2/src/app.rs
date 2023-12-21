use crate::config::Config;
use crate::process_atom::Process;
use crate::ready::Ready;

pub struct App {
    pub states: Handler<dyn Ready>,
    pub configs: Handler<dyn Config>,
}
pub struct Handler<T: ?Sized + Process> {
    pub items: Vec<Box<T>>,
}

impl<T: ?Sized + Process> Handler<T> {
    pub fn process(&self) {
        for item in &self.items {
            item.process();
        }
    }

    pub fn add(&mut self, item: Box<T>) {
        self.items.push(item);
    }
}
