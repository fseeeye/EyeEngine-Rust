use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::Window
};

use super::gpu::GPUState;


// ref: https://github.com/sotrh/learn-wgpu/blob/0.11/docs/beginner/
// ref: https://github.com/bevyengine/bevy/blob/669849c4547f1fd0950d7f03f56f78d4681db7f1/src/application.rs
pub trait Application {
    fn start(&self) {
        // When wgpu hits any error it panics with a generic message, while logging the real error via the env_logger crate. 
        // This means if you don't include env_logger::init() wgpu will fail silently, leaving you very confused!
        env_logger::init();

        // Build a Window event loop
        let event_loop = EventLoop::new();

        // Create a Window
        let window = Window::new(&event_loop).unwrap();

        // Init GPU States
        let mut state = pollster::block_on(GPUState::new(&window)); // await until it's done.

        // Event handling
        event_loop.run(move |event, _event_loop_window_target, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            // *control_flow = ControlFlow::Poll;

            match event {
                // A Window object can generate WindowEvents when certain input events occur,
                // such as a cursor moving over the window or a key getting pressed while the window is focused.
                // event ref: https://docs.rs/winit/0.26.0/winit/event/enum.Event.html#variant.WindowEvent
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == window.id() => if !state.input(event) { // if this Window Event isn't processed by GPUState::input()
                    match event {
                        // if get "window close" or "keyboard input `ESC`" event, end loop. 
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        // resized events: WindowEvent::Resized or WindowEvent::ScaleFactorChanged
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        },
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            state.resize(**new_inner_size)
                        }
                        // ignore other Window Event
                        _ => {}
                    }
                },
                // Emitted when all of the event loop’s input events have been processed and redraw processing is about to begin.
                // event ref: https://docs.rs/winit/0.26.0/winit/event/enum.Event.html#variant.MainEventsCleared
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually request it.
                    window.request_redraw();
                },
                // Emitted after MainEventsCleared **when a window should be redrawn**.
                // event ref: https://docs.rs/winit/0.26.0/winit/event/enum.Event.html#variant.RedrawRequested
                Event::RedrawEventsCleared => {
                    state.update();

                    match state.render() {
                        Ok(_) => {},
                        // Reconfigure the surface if lost.
                        Err(wgpu::SurfaceError::Lost) => {
                            println!("lost!");
                            state.resize(state.size)
                        },
                        // The System is out of memory, we should probably quit.
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e)
                    }
                },
                _ => {}
            }
        });
    }
    
    fn update(&self);
}
