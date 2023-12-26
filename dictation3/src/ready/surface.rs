use wgpu::{Instance, Surface};
use winit::window::Window;

use super::ReadyStatic;

impl ReadyStatic<(Instance, Window), Surface> for Surface {
    fn ready(wgpu_inst_and_window: (Instance, Window)) -> Surface {
        println!("Surface is ready");
        let inst = wgpu_inst_and_window.0;
        unsafe { inst.create_surface(&wgpu_inst_and_window.1).unwrap() }
    }
}
