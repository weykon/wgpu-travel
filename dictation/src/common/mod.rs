use super::app;

// ready 给大部分的new中继续实现
pub trait Ready {
    type Output;
    fn ready(app: &app::state::App) -> Self::Output;
}

// step, walk, quiet, watch, draw, listen,
pub trait Step {
    fn pre(&mut self) -> ();
    fn now(&mut self) -> ();
    fn post(&mut self) -> ();
}

// 需要什么，就告诉什么
pub trait Tell {
    fn tell(&self) -> ();
}
