mod app;
pub mod config;
pub mod process_atom;
pub mod ready;

use app::App;

use crate::app::Handler;
fn main() {
    println!("ok");
    let app = App {
        states: Handler { items: Vec::new() },
        configs: Handler { items: Vec::new() },
    };
}
