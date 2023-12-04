// ready 给大部分的new中继续实现
pub trait Ready {
    type Input;
    type Output;
    fn ready( input: Self::Input) -> Self::Output;
}

