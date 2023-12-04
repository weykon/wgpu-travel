use winit::{error::OsError, window::WindowBuilder};

use super::super::state;

use winit::window::Window;

use super::super::common::Ready;

impl Ready for Window {
    fn ready(app: Self::Input) -> Self::Output {
        Box::new(WindowBuilder::new().build(&app.event_loop))
    }
    type Input = state::App;
    type Output = Box<Result<Window, OsError>>;
}
