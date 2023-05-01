use std::iter;
use chrono::NaiveTime;
use egui::FontDefinitions;
use egui_demo_lib::DemoWindows;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::{Platform, PlatformDescriptor};
use winit::{
  event::*,
  window::{Window},
};
use crate::constants::{INITIAL_HEIGHT, INITIAL_WIDTH};


pub struct State {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,
  window: Window,
  egui_rpass: RenderPass,
  demo_app: DemoWindows,
  start_time: NaiveTime,
  pub egui_platform: Platform,
}

impl State {
  // Creating some of the wgpu types requires async code
  pub async fn new(window: Window) -> Self {
    let size = window.inner_size();
    // The instance is a handle to our GPU
    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      dx12_shader_compiler: Default::default(),
    });

    // # Safety
    //
    // The surface needs to live as long as the window that created it.
    // State owns the window so this should be safe.
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .unwrap();

    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: None,
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default(),
        },
        // Some(&std::path::Path::new("trace")), // Trace path
        None,
      )
      .await
      .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    // Shader code in this tutorial assumes an Srgb surface texture. Using a different
    // one will result all the colors comming out darker. If you want to support non
    // Srgb surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps.formats.iter()
      .copied()
      .filter(|f| f.is_srgb())
      .next()
      .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
    };
    surface.configure(&device, &config);

    let platform = Platform::new(PlatformDescriptor {
      physical_height: INITIAL_HEIGHT,
      physical_width: INITIAL_WIDTH,
      scale_factor: window.scale_factor(),
      font_definitions: FontDefinitions::default(),
      style: Default::default(),
    });


    let egui_rpass = RenderPass::new(&device, surface_format, 1);

    // Display the demo application that ships with egui.
    let demo_app = egui_demo_lib::DemoWindows::default();
    let start_time = chrono::Local::now().time();
    Self {
      surface,
      device,
      queue,
      config,
      size,
      window,
      start_time,
      demo_app,
      egui_rpass,
      egui_platform: platform,
    }
  }

  pub fn window(&self) -> &Window {
    &self.window
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }

  pub fn input(&mut self, event: &WindowEvent) -> bool {
    false
  }

  pub fn egui_input(&mut self, event: &Event<()>)-> bool{
      self.egui_platform.handle_event(&event);
      false
  }

  pub fn update(&mut self) {
    self.egui_platform.update_time(
      (chrono::Local::now().time() - self.start_time).num_milliseconds() as f64 / 1000.0,
    );
    self.egui_platform.begin_frame();
    self.demo_app.ui(&self.egui_platform.context());
  }

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let full_output = self.egui_platform.end_frame(Some(&self.window));
    let paint_jobs = self.egui_platform.context().tessellate(full_output.shapes);

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            }),
            store: true,
          },
        })],
        depth_stencil_attachment: None,
      });
    }
    let tdelta = full_output.textures_delta;
    self.egui_rpass
      .add_textures(&self.device, &self.queue, &tdelta)
      .expect("Failed to add textures");
    let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
      physical_width: self.size.width,
      physical_height: self.size.height,
      scale_factor: self.window.scale_factor() as f32,
    };
    self.egui_rpass
      .update_buffers(&self.device, &self.queue, &paint_jobs, &screen_descriptor);
    self.egui_rpass
      .execute(&mut encoder, &view, &paint_jobs, &screen_descriptor, None)
      .unwrap();
    self.queue.submit(iter::once(encoder.finish()));
    output.present();

    self.egui_rpass
      .remove_textures(tdelta)
      .expect("Failed to remove textures");
    Ok(())
  }
}
