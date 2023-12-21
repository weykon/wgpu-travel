use winit::window::Window;

use crate::{app::App, atom::ready::Ready};

impl Ready for Window {
    fn ready(&self, app: &App) {
        println!("Window is ready")
    }
}
