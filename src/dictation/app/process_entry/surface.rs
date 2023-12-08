use std::borrow::BorrowMut;

use winit::window::Window;

use crate::dictation::{
    app::state::{self, App},
    common::Ready,
};

use super::wgpu_instance;

impl Ready for wgpu::Surface {
    fn ready(app: &App) -> Self::Output {
        let inst = app.wgpu_instance.as_mut().unwrap();
        let window = app.window.as_mut().unwrap();
        let surface = unsafe { inst.create_surface(&window).unwrap() };

        Box::new(Some(surface))
    }
    type Output = Box<Option<wgpu::Surface>>;
    
}
