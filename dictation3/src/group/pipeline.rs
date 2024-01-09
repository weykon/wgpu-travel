use wgpu::{ PipelineLayout, ShaderModule, SurfaceConfiguration};

use crate::{
    atom::app::App,
    rear_asset::{obj_instance::InstanceRaw, vertex::Vertex},
};

use super::Group;

pub struct MPipeLineGroup {
    storage: Vec<wgpu::RenderPipeline>,
}

impl Group<(&App, PipelineLayout, ShaderModule, SurfaceConfiguration), ()> for MPipeLineGroup {
    fn add(&mut self, input: (&App, PipelineLayout, ShaderModule, SurfaceConfiguration)) -> () {
        let (app, redner_pipeline_layout, shader, surface_config) = input;
        self.storage
            .push(self.create(app, redner_pipeline_layout, shader, surface_config));
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
        redner_pipeline_layout: PipelineLayout,
        shader: ShaderModule,
        surface_config: SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let device = &app.adapter_storage.unwrap().device;
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&redner_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                // buffers 字段告诉 wgpu 要把什么类型的顶点数据传递给顶点着色器。我们会在顶点着色器中指定顶点，所以这里先留空
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                cull_mode: Some(wgpu::Face::Back),
                front_face: wgpu::FrontFace::Ccw, // 2.
                // 将此设置为 Fill 以外的任何值都要需要开启 Feature::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // 需要开启 Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // 需要开启 Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
}
