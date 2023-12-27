pub mod surface;

pub trait Config<P, O> {
     fn config(&mut self, input: P) -> O;
}

pub trait ConfigStatic<P, O> {
    fn config(input: P) -> O;
}
