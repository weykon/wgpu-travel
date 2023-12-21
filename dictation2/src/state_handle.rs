
pub trait StateHandler {
    fn ready(&self);
    fn add_ready(&mut self, state: Box<dyn Ready>);
}
