use winit::window::WindowBuilder;

use crate::dictation::app::state::App;

use winit::window::Window;

use super::super::common::Ready;

impl Ready for Window {
    fn ready(app: &App) -> Self::Output {
        let event_loop = app.event_loop.as_mut().unwrap();
        Box::new(Some(WindowBuilder::new().build(&event_loop).unwrap()))
    }
    type Output = Box<Option<Window>>;
}
