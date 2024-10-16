mod camera;
mod model;
mod resource;
mod scene;
mod texture;
mod renderer;
mod gpu;

use renderer::Renderer;
use scene::Scene;
use std::sync::Arc;
pub use std::time::{Duration, Instant};
use wgpu::ShaderStages;
use winit::{
  application::ApplicationHandler,
  dpi::PhysicalSize,
  event::WindowEvent,
  window::{Theme, Window},
};

#[derive(Default)]
pub struct App {
  window: Option<Arc<Window>>,
  renderer: Option<Renderer<'static>>,
  gui_state: Option<egui_winit::State>,
  last_render_time: Option<Instant>,
  last_size: (u32, u32),
  panels_visible: bool,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let mut attributes = Window::default_attributes();

    attributes = attributes.with_title("Standalone Winit/Wgpu Example");

    if let Ok(window) = event_loop.create_window(attributes) {
      let first_window_handle = self.window.is_none();
      let window_handle = Arc::new(window);
      self.window = Some(window_handle.clone());
      if first_window_handle {
        let inner_size = window_handle.inner_size();
        self.last_size = (inner_size.width, inner_size.height);

        let gui_context = egui::Context::default();
        gui_context.set_pixels_per_point(window_handle.scale_factor() as f32);
        let viewport_id = gui_context.viewport_id();
        let gui_state = egui_winit::State::new(
          gui_context,
          viewport_id,
          &window_handle,
          Some(window_handle.scale_factor() as _),
          Some(Theme::Dark),
          None,
        );

        let (width, height) = (
          window_handle.inner_size().width,
          window_handle.inner_size().height,
        );

        env_logger::init();
        let renderer =
          pollster::block_on(
            async move { Renderer::new(window_handle.clone(), width, height).await },
          );
        self.renderer = Some(renderer);

        self.gui_state = Some(gui_state);
        self.last_render_time = Some(Instant::now());
      }
    }
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    let (Some(gui_state), Some(renderer), Some(window), Some(last_render_time)) = (
      self.gui_state.as_mut(),
      self.renderer.as_mut(),
      self.window.as_ref(),
      self.last_render_time.as_mut(),
    ) else {
      return;
    };

    // Receive gui window event
    if gui_state.on_window_event(window, &event).consumed {
      return;
    }

    // If the gui didn't consume the event, handle it
    match event {
      WindowEvent::KeyboardInput {
        event:
          winit::event::KeyEvent {
            physical_key: winit::keyboard::PhysicalKey::Code(key_code),
            ..
          },
        ..
      } => {
        // Exit by pressing the escape key
        if matches!(key_code, winit::keyboard::KeyCode::Escape) {
          event_loop.exit();
        }
      }
      WindowEvent::Resized(PhysicalSize { width, height }) => {
        let (width, height) = ((width).max(1), (height).max(1));
        log::info!("Resizing renderer surface to: ({width}, {height})");
        renderer.resize(width, height);
        self.last_size = (width, height);
      }
      WindowEvent::CloseRequested => {
        log::info!("Close requested. Exiting...");
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
        let now = Instant::now();
        let delta_time = now - *last_render_time;
        *last_render_time = now;

        let gui_input = gui_state.take_egui_input(window);
        gui_state.egui_ctx().begin_pass(gui_input);

        let title = "Rust/Wgpu";
        if self.panels_visible {
          egui::TopBottomPanel::top("top").show(gui_state.egui_ctx(), |ui| {
            ui.horizontal(|ui| {
              ui.label("File");
              ui.label("Edit");
            });
          });

          egui::SidePanel::left("left").show(gui_state.egui_ctx(), |ui| {
            ui.heading("Scene Explorer");
            if ui.button("Click me!").clicked() {
              log::info!("Button clicked!");
            }
          });

          egui::SidePanel::right("right").show(gui_state.egui_ctx(), |ui| {
            ui.heading("Inspector");
            if ui.button("Click me!").clicked() {
              log::info!("Button clicked!");
            }
          });

          egui::TopBottomPanel::bottom("bottom").show(gui_state.egui_ctx(), |ui| {
            ui.heading("Assets");
            if ui.button("Click me!").clicked() {
              log::info!("Button clicked!");
            }
          });
        }

        egui::Window::new(title).show(gui_state.egui_ctx(), |ui| {
          ui.checkbox(&mut self.panels_visible, "Show Panels");
        });

        let egui_winit::egui::FullOutput {
          textures_delta,
          shapes,
          pixels_per_point,
          ..
        } = gui_state.egui_ctx().end_pass();

        let paint_jobs = gui_state.egui_ctx().tessellate(shapes, pixels_per_point);

        let screen_descriptor = {
          let (width, height) = self.last_size;
          egui_wgpu::ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: window.scale_factor() as f32,
          }
        };

        renderer.render_frame(screen_descriptor, paint_jobs, textures_delta, delta_time);
      }
      _ => (),
    }

    window.request_redraw();
  }
}




#[repr(C)]
#[derive(Default, Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct UniformBuffer {
  mvp: nalgebra_glm::Mat4,
}

struct UniformBinding {
  pub buffer: wgpu::Buffer,
  pub bind_group: wgpu::BindGroup,
  pub bind_group_layout: wgpu::BindGroupLayout,
}

impl UniformBinding {
  pub fn new(device: &wgpu::Device, visibility: ShaderStages, contents: &[u8]) -> Self {
    let buffer = wgpu::util::DeviceExt::create_buffer_init(
      device,
      &wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      },
    );

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
      label: Some("uniform_bind_group_layout"),
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: buffer.as_entire_binding(),
      }],
      label: Some("uniform_bind_group"),
    });

    Self {
      buffer,
      bind_group,
      bind_group_layout,
    }
  }
}
