use crate::app::App;

pub trait Process {
    fn process(&self, app: &App);
}
