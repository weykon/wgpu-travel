use crate::atom::app::App;

pub mod pipeline;
pub mod texture;

pub struct Groups {
    pub pipeline: pipeline::MPipeLineGroup,
    pub texture: texture::MTextureGroup,
}

pub trait Group<P, O> {
    fn add(&mut self, input: P) -> O;
}
