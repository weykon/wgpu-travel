pub mod pipeline;
pub mod texture;

pub trait Layout<Input, O> {
    fn add(&mut self, input: Input) -> O;
}

pub struct LayoutStorage {
    pub pipeline: pipeline::MPipeLineLayout,
    pub texture: texture::MTextureLayout,
}
