use crate::atom::app::App;

use super::Layout;

pub struct MPipeLineGroup {
    storage: Vec<wgpu::PipelineLayout>,
}

impl Layout for MPipeLineGroup {
    type Output = wgpu::PipelineLayout;
    fn add(&mut self, app: &App) {
        self.storage.push(self.create(app))
    }
}

impl MPipeLineGroup {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }
    pub fn create(&self, app: &App) -> wgpu::MPipeLineGroup {
        app.adapter_storage
            .as_ref()
            .unwrap()
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("main_pipeline_layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            })
    }
}
