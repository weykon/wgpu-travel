use crate::dictation::app::state::App;

use super::Config;

impl Config for App {
    type Input = ();
    type Output = ();
    fn fixed(&self, input: Self::Input) {
        let surface_config = self.surface.as_mut().unwrap().fixed(&self);
    }
}
