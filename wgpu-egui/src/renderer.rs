use crate::{gpu::Gpu, scene::Scene, UiState};

pub struct Renderer<'window> {
  pub gpu: Gpu<'window>,
  depth_texture_view: wgpu::TextureView,
  egui_renderer: egui_wgpu::Renderer,
  scene: Scene,
}

impl<'window> Renderer<'window> {
  const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

  pub async fn new(
    window: impl Into<wgpu::SurfaceTarget<'window>>,
    width: u32,
    height: u32,
    ui_state: UiState
  ) -> Self {
    let gpu = Gpu::new_async(window, width, height).await;
    let depth_texture_view = gpu.create_depth_texture(width, height);

    let egui_renderer = egui_wgpu::Renderer::new(
      &gpu.device,
      gpu.surface_config.format,
      Some(Self::DEPTH_FORMAT),
      1,
      false,
    );

    let scene = Scene::new(&gpu,  width, height, ui_state).await;

    Self {
      gpu,
      depth_texture_view,
      egui_renderer,
      scene,
    }
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    self.gpu.resize(width, height);
    self.depth_texture_view = self.gpu.create_depth_texture(width, height);
    self.scene.camera.resize(width,height);
  }

  pub fn render_frame(
    &mut self,
    screen_descriptor: egui_wgpu::ScreenDescriptor,
    paint_jobs: Vec<egui::epaint::ClippedPrimitive>,
    textures_delta: egui::TexturesDelta,
    ui_state: &UiState,
    delta_time: crate::Duration,
  ) {
    let delta_time = delta_time.as_secs_f32();

    self
      .scene
      .update(&self.gpu,  ui_state, delta_time);

    for (id, image_delta) in &textures_delta.set {
      self
        .egui_renderer
        .update_texture(&self.gpu.device, &self.gpu.queue, *id, image_delta);
    }

    for id in &textures_delta.free {
      self.egui_renderer.free_texture(id);
    }

    let mut encoder = self
      .gpu
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    self.egui_renderer.update_buffers(
      &self.gpu.device,
      &self.gpu.queue,
      &mut encoder,
      &paint_jobs,
      &screen_descriptor,
    );

    let surface_texture = self
      .gpu
      .surface
      .get_current_texture()
      .expect("Failed to get surface texture!");

    let surface_texture_view = surface_texture
      .texture
      .create_view(&wgpu::TextureViewDescriptor {
        label: wgpu::Label::default(),
        aspect: wgpu::TextureAspect::default(),
        format: Some(self.gpu.surface_format),
        dimension: None,
        base_mip_level: 0,
        mip_level_count: None,
        base_array_layer: 0,
        array_layer_count: None,
      });

    encoder.insert_debug_marker("Render scene");

    // This scope around the crate::render_pass prevents the
    // crate::render_pass from holding a borrow to the encoder,
    // which would prevent calling `.finish()` in
    // preparation for queue submission.
    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &surface_texture_view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.19,
              g: 0.24,
              b: 0.42,
              a: 1.0,
            }),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: &self.depth_texture_view,
          depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: wgpu::StoreOp::Store,
          }),
          stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
      });
      self.scene.render(&mut render_pass);

      self.egui_renderer.render(
        &mut render_pass.forget_lifetime(),
        &paint_jobs,
        &screen_descriptor,
      );
    }

    self.gpu.queue.submit(std::iter::once(encoder.finish()));
    surface_texture.present();
  }
}
