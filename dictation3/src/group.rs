use crate::atom::app::App;

pub mod pipeline;
pub mod texture;

pub struct Groups {
    pub pipeline: pipeline::MPipeLineGroup,
    pub texture: texture::MTextureGroup,
}

pub trait Group {
    type Output;
    fn add(&mut self, app: &App);
}
