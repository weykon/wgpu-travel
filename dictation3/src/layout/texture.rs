use crate::atom::app::App;

use super::Layout;

pub struct MTextureLayout {
    storage: Vec<wgpu::BindGroupLayout>,
}

impl Layout for MTextureLayout {
    type Output = wgpu::BindGroupLayout;

    fn add(&mut self, app: &App) {
        self.storage.push(self.create(app))
    }
}

impl MTextureLayout {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }
    pub fn create(&self, app: &App) -> wgpu::BindGroupLayout {
        app.adapter_storage
            .as_ref()
            .unwrap()
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            })
    }
}
