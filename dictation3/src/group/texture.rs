use wgpu::{BindGroup, BindGroupLayout, Sampler, Texture, TextureView};

use crate::atom::app::App;

use super::Group;
pub struct MTextureGroup {
    storage: Vec<wgpu::BindGroup>,
}

impl
    Group<
        (
            &App,
            &wgpu::BindGroupLayout,
            &wgpu::TextureView,
            &wgpu::Sampler,
        ),
        (),
    > for MTextureGroup
{
    fn add(
        &mut self,
        input: (
            &App,
            &wgpu::BindGroupLayout,
            &wgpu::TextureView,
            &wgpu::Sampler,
        ),
    ) -> () {
        self.storage
            .push(self.create(input.0, input.1, input.2, input.3));
    }
}

impl MTextureGroup {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }
    fn create(
        &self,
        app: &App,
        texture_bind_group_layout: &BindGroupLayout,
        texture_view: &TextureView,
        sampler: &Sampler,
    ) -> wgpu::BindGroup {
        app.adapter_storage
            .as_ref()
            .unwrap()
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            })
    }
}
