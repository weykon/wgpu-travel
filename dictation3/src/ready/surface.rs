use wgpu::{Instance, Surface};

use super::ReadyStatic;

impl ReadyStatic<(Instance,Window), Surface> for Surface {
    fn ready(wgpu_inst_and_window: ) -> Self {
        println!("Surface is ready");
        unsafe { wgpu_inst.create_surface(window)}
    }
}
