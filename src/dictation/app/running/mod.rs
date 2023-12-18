use self::camera_ctrl::CameraController;

use super::state::App;
mod camera;
mod camera_ctrl;

pub mod input;

pub struct Running {
    pub app: Option<&'static App>,
    pub camera: Option<camera::Camera>,
    pub controller: camera_ctrl::CameraController,
}

impl Running {
    pub fn run(app: &App) -> Self {
        Running {
            app: Some(app),
            camera: None,
            controller: CameraController::new(0.2),
        }
    }
}
