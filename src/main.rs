mod ammo;
mod texture;
use wgpu_learn::extra::update_diffuse_texture;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
mod instance;
use wgpu_learn::state::State;

fn main() {
    println!("Hello, Here's weykon's wgpu learning repo!");

    println!("start the process from wrapping in the pollster, ever not know what this, \n So I gotta go ask gpt ");
    println!("这是关于异步去执行的安全挂起，作者的解释说，state的初始化是异步的，所以来到了这个run这里需要用这个东西来处理异步函数下的事情，他比较轻量级，这个pollster");
    dbg!("试试使用这个dbg!");
    let debug_this_var = 32;
    dbg!("试试这个变量：{debug_this_var}");

    pollster::block_on(process());
}

// 看看这个函数名字是async的
async fn process() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    // 窗口设置
    let mut app_state = State::new(&window).await;

    // 事件遍历
    dbg!("事件遍历: 其实这里我是有个疑问，他的轮询是如何决定多久跑一次的，他的怎么样运作，和性能的事情有什么的。");

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !app_state.input(event) {
                match event {
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
                    WindowEvent::Resized(physical_size) => {
                        app_state.resize(*physical_size);
                    }
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
