mod example;
mod vertex;
mod wgpu_utils;
mod grid;
mod texture;

use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::{ WindowBuilder},
};
use crate::example::Example;
use crate::grid::Grid;


async fn run() {
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .build(&event_loop)
    .unwrap();

  let mut app = Example::new(&window).await;

  event_loop.run(move |event, _, control_flow| {
    match event {
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == window.id() => match event {
        WindowEvent::Resized(physical_size) => {
          app.resize(*physical_size);
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
          // new_inner_size is &mut so w have to dereference it twice
          app.resize(**new_inner_size);
        }
        WindowEvent::CursorMoved { .. } => {
          app.input(event);
        }
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput {
          input,
          ..
        } => {
          match input {
            KeyboardInput {
              state: ElementState::Pressed,
              virtual_keycode: Some(VirtualKeyCode::Escape),
              ..
            } => *control_flow = ControlFlow::Exit,
            _ => { app.input(event);},
          }
        }
        _ => *control_flow = ControlFlow::Wait,
      }
      Event::MainEventsCleared => {
        app.update();
        app.render();
        *control_flow = ControlFlow::Wait;
      }
      _ => *control_flow = ControlFlow::Wait,
    }
  });
}

fn main() {
  futures::executor::block_on(run());
}