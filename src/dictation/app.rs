pub mod config;
mod picture;
mod process_entry;
pub mod state;
use super::common;
pub mod rendering;
pub mod running;

pub fn run() {
    pollster::block_on(process_entry::process());
}
