use super::state::App;
mod camera;

pub mod input;

pub struct Running {
    app: &'static App,
    camera: camera::Camera,
}

impl Running {
    pub fn run(app: &App) -> Self {
        Running { app, camera::Camera {

        } }
    }
}
