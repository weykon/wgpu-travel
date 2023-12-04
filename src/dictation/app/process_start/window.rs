use winit::window::WindowBuilder;

use super::super::state;

use winit::window::Window;

use super::super::common::Ready;

impl Ready for Window {
    fn ready(app: &state::App) {
        Box::new(WindowBuilder::new().build(&app.event_loop).unwrap())
    }
    type Input = state::App;

    type Output=();
}

