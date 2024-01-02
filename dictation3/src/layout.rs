use crate::atom::app::App;

pub mod pipeline;
pub mod texture;

pub trait Layout {
    type Output;
    fn add(&mut self, app: &App);
}

pub struct LayoutStorage {
    pub pipeline: pipeline::MPipeLineLayout,
    pub texture: texture::MTextureLayout,
}
