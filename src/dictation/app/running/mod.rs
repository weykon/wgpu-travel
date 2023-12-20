use self::camera_ctrl::CameraController;

use super::state::App;
mod camera;
mod camera_ctrl;

pub mod input;

pub struct Running {
    pub camera: Option<camera::Camera>,
    pub controller: camera_ctrl::CameraController,
    pub window_size: winit::dpi::PhysicalSize<u32>,
}

impl Running {
    pub fn run(app: &App) -> Self {
        let size = app.window.as_mut().unwrap().inner_size();
        Running {
            camera: None,
            controller: CameraController::new(0.2),
            window_size: size,
        }
    }

    pub fn resize(&mut self, app: &mut App, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.window_size = new_size;
            app.config.unwrap().surface.width = new_size.width;
            app.config.unwrap().surface.height = new_size.height;
            app.surface.as_mut().unwrap().configure(
                &app.config.as_mut().unwrap().device,
                &app.config.unwrap().surface,
            )
        }
    }
}
