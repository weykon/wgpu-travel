pub trait Ready {
    type Input;
    type Output;
    fn ready(&self, input: Self::Input) -> Self::Output;
}
