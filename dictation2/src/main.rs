mod app;
pub mod atom;
pub mod handle;
pub mod process_atom;
pub mod ready;

use app::App;
use winit::event_loop::EventLoop;

use crate::handle::Handler;

fn main() {
    println!("ok");
    let mut app = App {
        states: Handler { items: Vec::new() },
        configs: Handler { items: Vec::new() },
    };

    app.states.add(Box::new(EventLoop::new()));

    app.ready();
}
