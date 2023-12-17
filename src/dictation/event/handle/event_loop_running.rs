use winit::event::Event;

use crate::dictation::app::state::App;

pub fn handle_events(event_loop: &EventLoop<()>, window: &winit::window::Window, app_state: &App) {
    let run = event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            // 这里这个取false过的，是如果不是摄像头移动就考虑接下来的东西。
            // 这里里面说的是wsad按键的接受
            if !input(event) {
                // 我在想如何在这里的代码整理一下，不用那么长，起码分一下策略什么的。
                app_event_handles::handle_any_input(event, control_flow, &mut app_state, &window);
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            app_state.update();
            match app_state.render() {
                Ok(_) => {}
                // 当展示平面的上下文丢失，就需重新配置
                Err(wgpu::SurfaceError::Lost) => app_state.resize(app_state.size),
                // 系统内存不足时，程序应该退出。
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // 所有其他错误（过期、超时等）应在下一帧解决
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // 除非我们手动请求，RedrawRequested 将只会触发一次。
            window.request_redraw();
        }
        _ => {}
    });
}
