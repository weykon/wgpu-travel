use winit::event_loop::EventLoop;

use crate::atom::event_storage::EventStorage;

use super::ReadyStatic;

impl ReadyStatic<(), Self> for EventStorage {
    fn ready(_: ()) -> Self {
        println!("EventLoop is ready");
        EventStorage {
            event_loop: EventLoop::new(),
        }
    }
}
