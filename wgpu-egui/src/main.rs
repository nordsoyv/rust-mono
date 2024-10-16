// use wgpu_egui::run;

use wgpu_egui::App;
// mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let event_loop = winit::event_loop::EventLoop::builder().build()?;
  event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
  let mut app = App::default();
  event_loop.run_app(&mut app)?;
  Ok(())
}