use crate::ammo::{self, Vertex, VERTICES};
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

pub struct State {
    // 表面理解一下
    // 平面， 设备信息，程序队列，配置，屏幕大小的信息
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
}

impl State {
    // 创建某些 wgpu 类型需要使用异步代码
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();
        // 命令队列 queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL 后端并不支持 wgpu 的所有功能，
                    // 所以如果要以 web 为构建目标，就必须禁用一些功能。
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // 追踪 API 调用路径
            )
            .await
            .unwrap();
        let caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            // usage 字段描述了 SurfaceTexture 如何被使用。RENDER_ATTACHMENT 指定将被用来渲染到屏幕的纹理（我们将在后面讨论更多的 TextureUsages 枚举值）。
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        // let modes = surface.get_capabilities(&adapter).present_modes;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../assets/shader.wgsl").into()),
        });

        let redner_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&redner_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                // buffers 字段告诉 wgpu 要把什么类型的顶点数据传递给顶点着色器。我们会在顶点着色器中指定顶点，所以这里先留空
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            // 图元
            // PrimitiveTopology::TriangleList 意味着每三个顶点组成一个三角形。
            // front_face 字段告诉 wgpu 如何确定三角形的朝向。
            //      FrontFace::Ccw 指定顶点的帧缓冲区坐标（framebuffer coordinates）按逆时针顺序给出的三角形为朝前（面向屏幕外）。
            // cull_mode 字段告诉 wgpu 如何做三角形剔除。
            //      CullMode::Back 指定朝后（面向屏幕内）的三角形会被剔除（不被渲染）。
            //      我们会在讨论缓冲区（Buffer）时详细介绍剔除问题。
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
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let num_vertices = VERTICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 这个get_current_texture是等待surface提供一个新的SurfaceTexture
        let output = self.surface.get_current_texture()?;
        // 拿到后给这里用
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // TextureView，上面的view，是一个纹理的视图，可以用来渲染。渲染是利用纹理视图和纹理交互。
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            // 此处开了一个作用域是为了，下面再次使用的encoder，
            // 不会被上面的encoder.begin_render_pass的可变引用占用
            // 这样在这个作用域drop掉后就可以在下面使用encoder了

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
