use crate::{app::App, process_atom::Process};

pub struct Handler<T: ?Sized + Process> {
    pub items: Vec<Box<T>>,
}

impl<T: ?Sized + Process> Handler<T> {
    pub fn process(&self, app: &App) {
        for item in &self.items {
            item.process(app);
        }
    }

    pub fn add(&mut self, item: Box<T>) {
        self.items.push(item);
    }
}
