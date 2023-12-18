use winit::{event::{WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}, event_loop::ControlFlow, window::Window};

use crate::dictation::app::state::App;

pub fn handle_any_input(
    event: &WindowEvent,
    control_flow: &mut ControlFlow,
    app_state: &mut App,
    window: &Window,
) {
    match event {
        // 这里应该不能是用dbg!，
        // 关闭和键盘输入
        WindowEvent::CloseRequested
        | WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
            ..
        } => *control_flow = ControlFlow::Exit,

        // 窗口大小改变
        WindowEvent::Resized(physical_size) => {
            app_state.window.resize(*physical_size);
        }

        // 用于更新纹理, 这里是我自己写的吧，写错了的
        // ei ? 似乎没有，上面说的是当那个样子的键盘事件才算
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
            ..
        } => {
            println!("Space key pressed!");
            let mut app_state = app_state;
            update_diffuse_texture::exec(&mut app_state);
            app_state.update();
            window.request_redraw();
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            // new_inner_size 是 &&mut 类型，因此需要解引用两次
            app_state.resize(**new_inner_size);
        }
        _ => {}
    }
}
