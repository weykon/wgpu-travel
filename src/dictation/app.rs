mod process_entry;
pub mod state;
pub mod config;
mod picture;
use super::common; 
pub mod running;

pub fn run() {
    pollster::block_on(process_entry::process());
}

