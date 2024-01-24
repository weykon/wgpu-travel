use wgpu::BindGroupLayout;

use crate::atom::app::App;

use super::Layout;

pub struct MPipeLineLayout {
    pub storage: Vec<wgpu::PipelineLayout>,
}

impl Layout<(&App, Vec<&BindGroupLayout>), ()> for MPipeLineLayout {
    fn add(&mut self, input: (&App, Vec<&BindGroupLayout>)) {
        let app = input.0;
        let asset_bind_group_layouts = input.1;
        self.storage
            .push(self.create(app, asset_bind_group_layouts));
    }
}

impl MPipeLineLayout {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }
    pub fn create(
        &self,
        app: &App,
        asset_bind_group_layouts: Vec<&BindGroupLayout>,
    ) -> wgpu::PipelineLayout {
        let device = &app.adapter_storage.unwrap().device;
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &asset_bind_group_layouts,
            push_constant_ranges: &[],
        })
    }
}
