mod camera;
mod model;
mod resource;
mod state;
mod texture;

use state::State;
use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::*,
  event_loop::{ActiveEventLoop, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::{Window, WindowId},
};

struct App<'a> {
  window: Option<Arc<Window>>,
  state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if self.window.is_none() {
      let window = Arc::new(
        event_loop
          .create_window(Window::default_attributes())
          .unwrap(),
      );
      self.window = Some(window.clone());
      let state = pollster::block_on(State::new(window.clone()));
      self.state = Some(state);
    }
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested
      | WindowEvent::KeyboardInput {
        event:
          KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::Escape),
            ..
          },
        ..
      } => event_loop.exit(),

      WindowEvent::Resized(physical_size) => {
        log::info!("physical_size: {physical_size:?}");
        self.state.as_mut().unwrap().resize(physical_size);
      }
      WindowEvent::RedrawRequested => {
        self.window.as_ref().unwrap().request_redraw();
        let state = self.state.as_mut().unwrap();
        state.update();
        match state.render() {
          Ok(_) => {}
          // Reconfigure the surface if it's lost or outdated
          Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
            state.resize(state.size())
          }
          // The system is out of memory, we should probably quit
          Err(wgpu::SurfaceError::OutOfMemory) => {
            log::error!("OutOfMemory");
            event_loop.exit();
          }

          // This happens when the a frame takes too long to present
          Err(wgpu::SurfaceError::Timeout) => {
            log::warn!("Surface timeout")
          }
        }
      }
      _ => (),
    }
  }
}

pub async fn run() {
  env_logger::init();
  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
  let mut app = App {
    window: None,
    state: None,
  };
  let _ = event_loop.run_app(&mut app);
  //let window = WindowBuilder::new().build(&event_loop).unwrap();

  // let mut state = State::new(&window).await;
  // let mut last_render_time = std::time::Instant::now(); // NEW!
  // let mut surface_configured = false;
  // event_loop
  //   .run(move |event, control_flow| {
  //     match event {
  //       Event::WindowEvent {
  //         ref event,
  //         window_id,
  //       } if window_id == state.window().id() && !state.input(event) => {
  //         // UPDATED!
  //         match event {
  //           WindowEvent::CloseRequested
  //           | WindowEvent::KeyboardInput {
  //             event:
  //               KeyEvent {
  //                 state: ElementState::Pressed,
  //                 physical_key: PhysicalKey::Code(KeyCode::Escape),
  //                 ..
  //               },
  //             ..
  //           } => control_flow.exit(),
  //           WindowEvent::Resized(physical_size) => {
  //             log::info!("physical_size: {physical_size:?}");
  //             surface_configured = true;
  //             state.resize(*physical_size);
  //           }
  //           WindowEvent::RedrawRequested => {
  //             // This tells winit that we want another frame after this one
  //             state.window().request_redraw();

  //             if !surface_configured {
  //               return;
  //             }
  //             let now = std::time::Instant::now();
  //             let dt = now - last_render_time;
  //             last_render_time = now;
  //             state.update(dt);
  //             match state.render() {
  //               Ok(_) => {}
  //               // Reconfigure the surface if it's lost or outdated
  //               Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
  //                 state.resize(state.size)
  //               }
  //               // The system is out of memory, we should probably quit
  //               Err(wgpu::SurfaceError::OutOfMemory) => {
  //                 log::error!("OutOfMemory");
  //                 control_flow.exit();
  //               }

  //               // This happens when the a frame takes too long to present
  //               Err(wgpu::SurfaceError::Timeout) => {
  //                 log::warn!("Surface timeout")
  //               }
  //             }
  //           }
  //           _ => {}
  //         }
  //       }
  //       Event::DeviceEvent {
  //         event: DeviceEvent::MouseMotion { delta },
  //         ..
  //       } => {
  //         if state.mouse_pressed {
  //           state.camera_controller.process_mouse(delta.0, delta.1);
  //         }
  //       }
  //       _ => {}
  //     }
  //   })
  //   .unwrap();
}
