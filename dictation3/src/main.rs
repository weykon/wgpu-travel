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
            handle_window_event(&event, control_flow, &mut app, &window);
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            handle_redraw_requested(&mut app, control_flow);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
pub mod atom;
pub mod ready;

fn handle_window_event(
    event: &winit::event::WindowEvent,
    control_flow: &mut winit::event_loop::ControlFlow,
    app_state: &mut App,
    window: &Window,
) {
    if !app_state.input(event) {
        app_event_handles::handle_any_input(event, control_flow, app_state, window);
    }
}
fn handle_redraw_requested(
    app_state: &mut State,
    control_flow: &mut winit::event_loop::ControlFlow,
) {
    app_state.update();
    match app_state.render() {
        Ok(_) => {}
        Err(wgpu::SurfaceError::Lost) => app_state.resize(app_state.size),
        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
        Err(e) => eprintln!("{:?}", e),
    }
}
