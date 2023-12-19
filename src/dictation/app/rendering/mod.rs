use super::state::App;

pub mod app;

pub trait Rendering {
     fn render();
}

pub fn render(app: &mut App) {
    app.render();
}
