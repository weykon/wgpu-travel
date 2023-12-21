use crate::{app::App, process_atom::Process};

pub trait Config {
    fn config(&self, app: &App);
}

impl Process for dyn Config {
    fn process(&self, app: &App) {
        // 实现 config 的处理逻辑
        self.config(app);
    }
}
