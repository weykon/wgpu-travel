use super::my::Ready;
use winit::window::Window;

impl Ready for Window {
    type Output = Window;
    fn ready(&self, input: Self::Input) -> Self::Output {
        Windowbuilder::new().build(&event_loop)
    }
}
