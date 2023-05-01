use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::{WindowBuilder},
};
use crate::app::State;
use crate::constants::{INITIAL_HEIGHT, INITIAL_WIDTH};

mod app;
mod constants;


async fn run() {
  env_logger::init();

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_decorations(true)
    .with_resizable(true)
    .with_title("Landscape example")
    .with_inner_size(winit::dpi::PhysicalSize {
      height: INITIAL_HEIGHT,
      width: INITIAL_WIDTH,
    })
    .build(&event_loop)
    .unwrap();

  // State::new uses async code, so we're going to wait for it to finish
  let mut state = State::new(window).await;

  event_loop.run(move |event, _, control_flow| {
    state.egui_input(&event);
    // state.egui_platform.handle_event(&event);
    match event {
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == state.window().id() => {
        if !state.input(event) {
          // UPDATED!
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
              state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
              // new_inner_size is &&mut so w have to dereference it twice
              state.resize(**new_inner_size);
            }
            _ => {}
          }
        }
      }
      Event::RedrawRequested(window_id) if window_id == state.window().id() => {
        state.update();
        match state.render() {
          Ok(_) => {}
          // Reconfigure the surface if it's lost or outdated
          Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
          // The system is out of memory, we should probably quit
          Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

          Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
        }
      }
      Event::RedrawEventsCleared => {
        // RedrawRequested will only trigger once, unless we manually
        // request it.
        state.window().request_redraw();
      }
      _ => {}
    }
  });
}


fn main() {
  pollster::block_on(run());
}