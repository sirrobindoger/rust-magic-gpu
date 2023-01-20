use std::time::{Instant, Duration};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod state;
mod vertex;
mod texture;
mod camera;
mod cameracontroller;
mod engine;

use state::State;
use log::debug;

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;
    let mut next_draw = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let now = Instant::now();

        match next_draw.checked_duration_since(now) {
            Some(next_draw) => {
                *control_flow = ControlFlow::WaitUntil(now + next_draw);
            }

            None => {
                next_draw = now + Duration::from_millis(16);
                *control_flow = ControlFlow::WaitUntil(next_draw);

                window.request_redraw();
            }
        }

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !state.input(event){ 
                match event {
                    WindowEvent::CloseRequested |WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }

                    _ => (),
                    }
                },
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                },
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    window.request_redraw();
                }
                
             _ => (),
        }
        });
}