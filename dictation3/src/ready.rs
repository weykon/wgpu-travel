pub mod window;
pub mod app;
pub mod event_storage;

pub trait Ready<P, O> {
    fn ready(&mut self, input: P) -> O;
}

pub trait ReadyStatic<P, O> {
    fn ready(input: P) -> O;
}