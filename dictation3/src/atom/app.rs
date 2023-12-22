use std::cell::RefCell;

use winit::window::Window;

use super::event_storage::EventStorage;

pub struct App {
    pub event_storage: RefCell<EventStorage>,
    pub window: RefCell<Window>,
}
