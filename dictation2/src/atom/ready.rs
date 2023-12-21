use crate::{app::App, process_atom::Process};

pub trait Ready {
    fn ready(&self, app: &App);
}

impl Process for dyn Ready {
    fn process(&self, app: &App) {
        self.ready(app)
    }
}
