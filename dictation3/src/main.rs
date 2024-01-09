use std::cell::RefCell;

use wgpu::{Instance, Surface};
use winit::{event::Event, window::Window};

use crate::{
    atom::{
        adapter::{self},
        app::App,
        event_storage::EventStorage,
    },
    config::Config,
    group::Group,
    layout::Layout,
    ready::ReadyStatic,
};

fn main() {
    println!("Hello, world!");

    // ready
    let mut event = EventStorage::ready(());
    let window = Window::ready(&event);
    let wgpu_inst = Instance::ready(());
    let surface = Surface::ready((wgpu_inst, window));
    let adapter_storage = adapter::AdapterStorage::ready((wgpu_inst, surface));

    let app = App {
        event_storage: RefCell::new(event),
        window: RefCell::new(window),
        surface: RefCell::new(surface),
        adapter_storage: Some(Box::new(adapter_storage)),
        surface_config: None,
    };

    // config
    app.surface.borrow_mut().config((&adapter_storage, &window));

    // layout
    let mut texture_layouts = layout::texture::MTextureLayout::new();
    let mut pipeline_layouts = layout::pipeline::MPipeLineLayout::new();
    texture_layouts.add(&app);

    pipeline_layouts.add((&app, texture_layouts.storage.iter().collect()));

    // prepare a asset
    let device = &adapter_storage.device;
    let queue = &adapter_storage.queue;
    let texture_bytes = include_bytes!("../../assets/tree.png");
    let texture_storage =
        asset::texture::Texture::from_bytes(&device, &queue, texture_bytes, "../assets/tree.png")
            .unwrap();

    // group
    let layout_storage = layout::LayoutStorage {
        texture: texture_layouts,
        pipeline: pipeline_layouts,
    };

    let mut texture_groups = group::texture::MTextureGroup::new();

    let a_texture_group = texture_layouts.storage[0];
    let texture_group = texture_groups.add((
        &app,
        &a_texture_group,
        &texture_storage.view,
        &texture_storage.sampler,
    ));

    event.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, window_id } if window_id == window.id() => {
            handle::handle_window_event(&event, control_flow, &mut app, &window);
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            handle::handle_redraw_requested(&mut app, control_flow);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
pub mod asset;
pub mod atom;
pub mod config;
pub mod group;
pub mod handle;
pub mod layout;
pub mod ready;
pub mod rear_asset;
