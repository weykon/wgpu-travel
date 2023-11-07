use winit::{event_loop::EventLoop, window::WindowBuilder};

use super::event::handle_events;

pub fn run() {
    pollster::block_on(process());
}

async fn process() -> () {
    // window and event
    let event_loop = EventLoop::new();
    let mut window = WindowBuilder::new().build(&event_loop);

    handle_events(&event_loop);




}

