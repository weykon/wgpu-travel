use crate::layout;

pub struct Scheduler {
    pub pipeline_layouts: Option<layout::pipeline::MPipeLineLayout>,
    pub texture_layouts: Option<layout::texture::MTextureLayout>,
}
