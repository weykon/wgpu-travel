use std::sync::Arc;

use wgpu::{Instance, Surface};

pub struct AppView {
    pub window: winit::window::Window,
    pub maximum_frames: i32,
    pub scale_factor: f32,
    pub sdq: SurfaceDeviceQueue,
    pub callback_to_app: Option<extern "C" fn(arg: i32)>,
    pub temporary_directory: &'static str,
    pub library_directory: &'static str,
}

impl AppView {
    pub async fn new(window: winit::window::Window) -> Self {
        let scale_factor = window.scale_factor();
        // 如果配置在webgl，去启用wgpu下的多WebGL后端
        let default_backends = if cfg!(feature = "webgl") {
            wgpu::Backends::GL
        } else {
            wgpu::Backends::PRIMARY
        };
        let backends = wgpu::util::backend_bits_from_env().unwrap_or(default_backends);

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });
        let physical_size = window.inner_size();

        let surface = unsafe {
            #[cfg(target_arch = "wasm32")]
            let surface = {
                use winit::platform::web::WindowExtWebSys;
                instance.create_surface_from_canvas(window.canvas())
            };
            #[cfg(not(target_arch = "wasm32"))]
            let surface = instance.create_surface(&window);
            match surface {
                Ok(surface) => surface,
                Err(e) => panic!("Failed to create surface: {e:?}"),
            }
        };

        let (adapter, device, queue) = request_device(&instance, &surface).await;

        let caps = surface.get_capabilities(&adapter);

        let modes = caps.alpha_modes;
        let alpha_mode = match modes.iter().find(|&&m| {
            m == wgpu::CompositeAlphaMode::PreMultiplied
                || m == wgpu::CompositeAlphaMode::PostMultiplied
                || m == wgpu::CompositeAlphaMode::Inherit
        }) {
            Some(&wgpu::CompositeAlphaMode::PreMultiplied) => {
                wgpu::CompositeAlphaMode::PreMultiplied
            }
            Some(&wgpu::CompositeAlphaMode::PostMultiplied) => {
                wgpu::CompositeAlphaMode::PostMultiplied
            }
            Some(&wgpu::CompositeAlphaMode::Inherit) => wgpu::CompositeAlphaMode::Inherit,
            _ => modes[0],
        };

        let prefered = caps.formats[0];
        let format = if cfg!(all(target_arch = "wasm32", not(feature = "webgl"))) {
            // Chorme Webgpu 不支持sRGB
            prefered.remove_srgb_suffix()
        } else {
            prefered
        };

        let view_formats = if cfg!(feature = "webgl") {
            vec![]
        } else {
            vec![format.add_srgb_suffix(), format.remove_srgb_suffix()]
        };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode,
            view_formats,
        };
        surface.configure(&device, &config);

        AppSurface {
            window,
            scale_factor: scale_factor as f32,
            maximum_frames: 60,
            sdq: SurfaceDeviceQueue {
                surface,
                config,
                adapter,
                device: Arc::new(device),
                queue: Arc::new(queue),
            },
            callback_to_app: None,
            temporary_directory: "",
            library_directory: "",
        }
    }
}
struct SurfaceDeviceQueue {
    pub surface: Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}
impl SurfaceDeviceQueue {
    pub fn update_config_format(&mut self, format: wgpu::TextureFormat) {
        self.config.format = format;
        if cfg!(feature = "webgl") {
            // webgl 后端不支持 view_formats
        } else {
            // 颜色空间
            self.config.view_formats = vec![format.add_srgb_suffix(), format.remove_srgb_suffix()];
        }
        self.surface.configure(&self.device, &self.config);
    }
}

#[allow(unused)]
async fn request_device(
    instance: &Instance,
    surface: &Surface,
) -> (wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    // get adapter
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::util::power_preference_from_env()
                .unwrap_or(wgpu::PowerPreference::HighPerformance),
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await
        .expect("No suitable GPU adapters found on the system!");

    let adapter_info = adapter.get_info();
    println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

    // api
    let res = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: adapter.features(),
                limits: adapter.limits(),
            },
            None,
        )
        .await;
    match res {
        Err(err) => {
            panic!("request_device failed: {err:?}");
        }
        Ok(tuple) => (adapter, tuple.0, tuple.1),
    }
}

fn main() {}
