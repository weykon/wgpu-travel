use std::cell::RefCell;

use winit::{event::Event, window::Window};

use crate::{
    atom::{app::App, event_storage::EventStorage},
    ready::ReadyStatic,
};

fn main() {
    println!("Hello, world!");

    let mut event = EventStorage::ready(());
    let window = Window::ready(&event);
    let app = App {
        event_storage: RefCell::new(event),
        window: RefCell::new(window),
    };

    event.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, window_id } if window_id == window.id() => {
            handle::handle_window_event(&event, control_flow, &mut app, &window);
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            handle::handle_redraw_requested(&mut app, control_flow);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
pub mod atom;
pub mod ready;
pub mod handle;