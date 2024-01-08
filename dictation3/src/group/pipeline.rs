use wgpu::BindGroupDescriptor;

use crate::atom::app::App;

use super::Layout;

pub struct MPipeLineGroup {
    storage: Vec<wgpu::PipelineLayout>,
}

impl Group for MPipeLineGroup {
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
    pub fn create(
        &self,
        app: &App,
        &bind_group_layout: wgpu::BindGroupLayout,
        &texture_bind_group: wgpu::BindGroup,
        &texture_sampler: wgpu::BindingResource,
    ) -> MPipeLineGroup {
        app.adapter_storage
            .as_ref()
            .unwrap()
            .device
            .create_bind_group(&BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_bind_group),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture_sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            })
    }
}
