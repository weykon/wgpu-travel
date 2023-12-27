use std::cell::RefCell;

use wgpu::{Instance, Surface};
use winit::{event::Event, window::Window};

use crate::{
    atom::{
        adapter::{self, AdapterStorage},
        app::App,
        event_storage::EventStorage,
    },
    config::Config,
    ready::ReadyStatic,
};

fn main() {
    println!("Hello, world!");

    let mut event = EventStorage::ready(());
    let window = Window::ready(&event);
    let wgpu_inst = Instance::ready(());
    let surface = Surface::ready((wgpu_inst, window));
    let adapter_storage = adapter::AdapterStorage::ready((wgpu_inst, surface));

    let app = App {
        event_storage: RefCell::new(event),
        window: RefCell::new(window),
        surface: RefCell::new(surface),
        adapter_storage: Some(Box::new(adapter_storage)),
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

    app.surface.borrow_mut().config((&adapter_storage, &window));
}
pub mod atom;
pub mod config;
pub mod handle;
pub mod ready;
