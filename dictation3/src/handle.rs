use super::atom::app::App;
use winit::window::Window;

pub fn handle_window_event(
    event: &winit::event::WindowEvent,
    control_flow: &mut winit::event_loop::ControlFlow,
    app_state: &mut App,
    window: &Window,
) {
    if !app_state.input(event) {
        // app_event_handles::handle_any_input(event, control_flow, app_state, window);
    }
}
pub fn handle_redraw_requested(
    app_state: &mut App,
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
