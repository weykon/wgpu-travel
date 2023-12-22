use winit::window::{Window, WindowBuilder};

use crate::atom::event_storage::EventStorage;

use super::ReadyStatic;

impl ReadyStatic<&EventStorage, Window> for Window {
    fn ready(input: &EventStorage) -> Window {
        println!("Window is ready");
        WindowBuilder::new().build(&input.event_loop).unwrap()
    }
}
