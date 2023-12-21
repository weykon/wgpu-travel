use winit::event_loop::EventLoop;

use crate::{app::App, atom::ready::Ready};

impl Ready for EventLoop<()> {
    fn ready(&self, app: &App) {
        println!("EventLoop is ready")
        
    }
}
