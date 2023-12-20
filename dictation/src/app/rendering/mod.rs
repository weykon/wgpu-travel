use super::state::App;

pub mod app;

pub trait Rendering {
    type Input;
    type Output;
    fn render(&self, input: Self::Input) -> Self::Output;
}

pub fn render(app: &mut App) -> Result<(), wgpu::SurfaceError> {
    App::render(app, ())
}
