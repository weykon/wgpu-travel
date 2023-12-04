use crate::dictation::{common::Ready, app::state};

impl Ready for wgpu::Surface {
    fn ready(app: &state::App) {
        Box::new(app.window.as_ref().unwrap().create_surface(&app.event_loop))
    }
    type Input = state::App;

    type Output=Box<wgpu::Surface>;
}
