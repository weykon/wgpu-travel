use wgpu::{BindGroupLayout, Texture};

use crate::atom::app::App;

use super::Group;

pub struct MTextureGroup {
    storage: Vec<wgpu::BindGroup>,
}

impl Group for MTextureGroup {
    type Output = wgpu::BindGroup;

    fn add(&mut self, app: &App) {
        self.storage.push(self.create(app))
    }
}

impl MTextureGroup {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }
    pub fn create(&self, app: &App, texture_bind_group_layout: BindGroupLayout,texture: Texture) -> wgpu::BindGroup {
        app.adapter_storage
            .as_ref()
            .unwrap()
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            })
    }
}
