mod camera;
mod model;
mod resource;
mod texture;

use camera::{Camera, CameraController, CameraUniform, Projection};
use cgmath::prelude::*;
use model::{DrawModel, Vertex};
use std::sync::Arc;
pub use std::time::{Duration, Instant};
use wgpu::{util::DeviceExt, ShaderStages};
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

pub struct Renderer<'window> {
  gpu: Gpu<'window>,
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

    let scene = Scene::new(&gpu.device, gpu.surface_format, &gpu.queue, width, height).await;

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
  }

  pub fn render_frame(
    &mut self,
    screen_descriptor: egui_wgpu::ScreenDescriptor,
    paint_jobs: Vec<egui::epaint::ClippedPrimitive>,
    textures_delta: egui::TexturesDelta,
    delta_time: crate::Duration,
  ) {
    let delta_time = delta_time.as_secs_f32();

    self
      .scene
      .update(&self.gpu.queue, self.gpu.aspect_ratio(), delta_time);

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

pub struct Gpu<'window> {
  pub surface: wgpu::Surface<'window>,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub surface_config: wgpu::SurfaceConfiguration,
  pub surface_format: wgpu::TextureFormat,
}

impl<'window> Gpu<'window> {
  pub fn aspect_ratio(&self) -> f32 {
    self.surface_config.width as f32 / self.surface_config.height.max(1) as f32
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    self.surface_config.width = width;
    self.surface_config.height = height;
    self.surface.configure(&self.device, &self.surface_config);
  }

  pub fn create_depth_texture(&self, width: u32, height: u32) -> wgpu::TextureView {
    let texture = self.device.create_texture(
      &(wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size: wgpu::Extent3d {
          width,
          height,
          depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
      }),
    );
    texture.create_view(&wgpu::TextureViewDescriptor {
      label: None,
      format: Some(wgpu::TextureFormat::Depth32Float),
      dimension: Some(wgpu::TextureViewDimension::D2),
      aspect: wgpu::TextureAspect::All,
      base_mip_level: 0,
      base_array_layer: 0,
      array_layer_count: None,
      mip_level_count: None,
    })
  }

  pub async fn new_async(
    window: impl Into<wgpu::SurfaceTarget<'window>>,
    width: u32,
    height: u32,
  ) -> Self {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all),
      ..Default::default()
    });

    let surface = instance.create_surface(window).unwrap();

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .expect("Failed to request adapter!");
    let (device, queue) = {
      log::info!("WGPU Adapter Features: {:#?}", adapter.features());
      adapter
        .request_device(
          &wgpu::DeviceDescriptor {
            label: Some("WGPU Device"),

            required_features: wgpu::Features::default(),
            required_limits: wgpu::Limits {
              max_texture_dimension_2d: 4096, // Allow higher resolutions on native
              ..wgpu::Limits::downlevel_defaults()
            },
            memory_hints: wgpu::MemoryHints::default(),
          },
          None,
        )
        .await
        .expect("Failed to request a device!")
    };

    let surface_capabilities = surface.get_capabilities(&adapter);

    // This assumes an sRGB surface texture
    let surface_format = surface_capabilities
      .formats
      .iter()
      .copied()
      .find(|f| !f.is_srgb()) // egui wants a non-srgb surface texture
      .unwrap_or(surface_capabilities.formats[0]);

    let surface_config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width,
      height,
      present_mode: surface_capabilities.present_modes[0],
      alpha_mode: surface_capabilities.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &surface_config);

    Self {
      surface,
      device,
      queue,
      surface_config,
      surface_format,
    }
  }
}

const NUM_INSTANCES_PER_ROW: u32 = 10;
const SPACE_BETWEEN: f32 = 3.0;

struct Scene {
  //   pub model: nalgebra_glm::Mat4,
  //   pub vertex_buffer: wgpu::Buffer,
  //   pub index_buffer: wgpu::Buffer,
  //   pub uniform: UniformBinding,
  //   pub pipeline: wgpu::RenderPipeline,
  pub camera: Camera,
  pub projection: Projection,
  pub camera_controller: CameraController,
  camera_uniform_binding: UniformBinding,
  camera_uniform: CameraUniform,
  render_pipeline: wgpu::RenderPipeline,
  light_render_pipeline: wgpu::RenderPipeline,
  obj_model: model::Model,
  instances: Vec<Instance>,
  instance_buffer: wgpu::Buffer,
  diffuse_bind_group: wgpu::BindGroup,
  light_uniform_binding: UniformBinding,
  light_uniform: LightUniform,
}

impl Scene {
  pub async fn new(
    device: &wgpu::Device,
    surface_format: wgpu::TextureFormat,
    queue: &wgpu::Queue,
    width: u32,
    height: u32,
  ) -> Self {
    let diffuse_bytes = include_bytes!("happy-tree.png");
    let diffuse_texture =
      texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy_tree.png").unwrap();
    let texture_bind_group_layout =
      device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
              multisampled: false,
              view_dimension: wgpu::TextureViewDimension::D2,
              sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
          },
          wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            // This should match the filterable field of the
            // corresponding Texture entry above.
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
          },
        ],
        label: Some("texture_bind_group_layout"),
      });
    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &texture_bind_group_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
        },
      ],
      label: Some("diffuse_bind_group"),
    });

    let camera = Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
    let projection = Projection::new(width, height, cgmath::Deg(45.0), 0.1, 100.0);
    let camera_controller = CameraController::new(4.0, 2.0);
    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera, &projection);
    let camera_uniform_binding = UniformBinding::new(
      device,
      ShaderStages::VERTEX_FRAGMENT,
      bytemuck::cast_slice(&[camera_uniform]),
    );

    let light_uniform = LightUniform {
      position: [2.0, 2.0, 2.0],
      _padding: 0,
      color: [1.0, 1.0, 1.0],
      _padding2: 0,
    };
    let light_uniform_binding = UniformBinding::new(device, ShaderStages::VERTEX_FRAGMENT,bytemuck::cast_slice(&[light_uniform]));

    let instances = (0..NUM_INSTANCES_PER_ROW)
      .flat_map(|z| {
        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
          let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
          let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

          let position = cgmath::Vector3 { x, y: 0.0, z };

          let rotation = if position.is_zero() {
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
          } else {
            cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
          };

          Instance { position, rotation }
        })
      })
      .collect::<Vec<_>>();
    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Instance Buffer"),
      contents: bytemuck::cast_slice(&instance_data),
      usage: wgpu::BufferUsages::VERTEX,
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[
        &texture_bind_group_layout,
        &camera_uniform_binding.bind_group_layout,
        &light_uniform_binding.bind_group_layout,
      ],
      push_constant_ranges: &[],
    });

    let render_pipeline = {
      let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Normal Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
      };
      create_render_pipeline(
        &device,
        &render_pipeline_layout,
        surface_format,
        Some(texture::Texture::DEPTH_FORMAT),
        &[model::ModelVertex::desc(), InstanceRaw::desc()],
        shader,
      )
    };

    let light_render_pipeline = {
      let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Light Pipeline Layout"),
        bind_group_layouts: &[
          &camera_uniform_binding.bind_group_layout,
          &light_uniform_binding.bind_group_layout,
        ],
        push_constant_ranges: &[],
      });
      let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Light Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
      };
      create_render_pipeline(
        &device,
        &layout,
        surface_format,
        Some(texture::Texture::DEPTH_FORMAT),
        &[model::ModelVertex::desc()],
        shader,
      )
    };

    let obj_model = resource::load_model("cube.obj", &device, &queue, &texture_bind_group_layout)
      .await
      .unwrap();

    Self {
      //   model: nalgebra_glm::Mat4::identity(),
      //   uniform,
      render_pipeline,
      light_render_pipeline,
      //   vertex_buffer,
      //   index_buffer,
      camera,
      projection,
      camera_controller,
      camera_uniform_binding,
      camera_uniform,
      obj_model,
      instances,
      instance_buffer,
      diffuse_bind_group,
      light_uniform_binding,
      light_uniform,
    }
  }

  pub fn render<'rpass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'rpass>) {
    // renderpass.set_pipeline(&self.pipeline);
    // renderpass.set_bind_group(0, &self.uniform.bind_group, &[]);

    // renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    // renderpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

    // renderpass.draw_indexed(0..(INDICES.len() as _), 0, 0..1);

    render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
    render_pass.set_bind_group(1, &self.camera_uniform_binding.bind_group, &[]);
    render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

    use crate::model::DrawLight; // NEW!
    render_pass.set_pipeline(&self.light_render_pipeline); // NEW!
    render_pass.draw_light_model(
      &self.obj_model,
      &self.camera_uniform_binding.bind_group,
      &self.light_uniform_binding.bind_group,
    ); // NEW!

    render_pass.set_pipeline(&self.render_pipeline); // 2.

    render_pass.draw_model_instanced(
      &self.obj_model,
      0..self.instances.len() as u32,
      &self.camera_uniform_binding.bind_group,
      &self.light_uniform_binding.bind_group,
    );
  }

  pub fn update(&mut self, queue: &wgpu::Queue, aspect_ratio: f32, delta_time: f32) {
    self
      .camera_controller
      .update_camera(&mut self.camera, delta_time);
    self
      .camera_uniform
      .update_view_proj(&self.camera, &self.projection);
    queue.write_buffer(
      &self.camera_uniform_binding.buffer,
      0,
      bytemuck::cast_slice(&[self.camera_uniform]),
    );

    let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
    self.light_uniform.position =
      (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(60.0 * delta_time))
        * old_position)
        .into();
    queue.write_buffer(
      &self.light_uniform_binding.buffer,
      0,
      bytemuck::cast_slice(&[self.light_uniform]),
    );

    // let projection =
    //   nalgebra_glm::perspective_lh_zo(aspect_ratio, 80_f32.to_radians(), 0.1, 1000.0);
    // let view = nalgebra_glm::look_at_lh(
    //   &nalgebra_glm::vec3(0.0, 0.0, 3.0),
    //   &nalgebra_glm::vec3(0.0, 0.0, 0.0),
    //   &nalgebra_glm::Vec3::y(),
    // );
    // self.model = nalgebra_glm::rotate(
    //   &self.model,
    //   30_f32.to_radians() * delta_time,
    //   &nalgebra_glm::Vec3::y(),
    // );
    // self.uniform.update_buffer(
    //   queue,
    //   0,
    //   UniformBuffer {
    //     mvp: projection * view * self.model,
    //   },
    // );
  }

  //   fn create_pipeline(
  //     device: &wgpu::Device,
  //     surface_format: wgpu::TextureFormat,
  //     uniform: &UniformBinding,
  //   ) -> wgpu::RenderPipeline {
  //     let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
  //       label: None,
  //       source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(SHADER_SOURCE)),
  //     });

  //     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
  //       label: None,
  //       bind_group_layouts: &[&uniform.bind_group_layout],
  //       push_constant_ranges: &[],
  //     });

  //     device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
  //       label: None,
  //       layout: Some(&pipeline_layout),
  //       vertex: wgpu::VertexState {
  //         module: &shader_module,
  //         entry_point: "vertex_main",
  //         buffers: &[Vertex::description(&Vertex::vertex_attributes())],
  //         compilation_options: Default::default(),
  //       },
  //       primitive: wgpu::PrimitiveState {
  //         topology: wgpu::PrimitiveTopology::TriangleStrip,
  //         strip_index_format: Some(wgpu::IndexFormat::Uint32),
  //         front_face: wgpu::FrontFace::Cw,
  //         cull_mode: None,
  //         polygon_mode: wgpu::PolygonMode::Fill,
  //         conservative: false,
  //         unclipped_depth: false,
  //       },
  //       depth_stencil: Some(wgpu::DepthStencilState {
  //         format: Renderer::DEPTH_FORMAT,
  //         depth_write_enabled: true,
  //         depth_compare: wgpu::CompareFunction::Less,
  //         stencil: wgpu::StencilState::default(),
  //         bias: wgpu::DepthBiasState::default(),
  //       }),
  //       multisample: wgpu::MultisampleState {
  //         count: 1,
  //         mask: !0,
  //         alpha_to_coverage_enabled: false,
  //       },
  //       fragment: Some(wgpu::FragmentState {
  //         module: &shader_module,
  //         entry_point: "fragment_main",
  //         targets: &[Some(wgpu::ColorTargetState {
  //           format: surface_format,
  //           blend: Some(wgpu::BlendState::ALPHA_BLENDING),
  //           write_mask: wgpu::ColorWrites::ALL,
  //         })],
  //         compilation_options: Default::default(),
  //       }),
  //       multiview: None,
  //       cache: None,
  //     })
  //   }
}

// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// struct Vertex {
//   position: [f32; 4],
//   color: [f32; 4],
// }

// impl Vertex {
//   pub fn vertex_attributes() -> Vec<wgpu::VertexAttribute> {
//     wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4].to_vec()
//   }

//   pub fn description(attributes: &[wgpu::VertexAttribute]) -> wgpu::VertexBufferLayout {
//     wgpu::VertexBufferLayout {
//       array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
//       step_mode: wgpu::VertexStepMode::Vertex,
//       attributes,
//     }
//   }
// }

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

  pub fn update_buffer(
    &mut self,
    queue: &wgpu::Queue,
    offset: wgpu::BufferAddress,
    uniform_buffer: UniformBuffer,
  ) {
    queue.write_buffer(
      &self.buffer,
      offset,
      bytemuck::cast_slice(&[uniform_buffer]),
    )
  }
}

// const VERTICES: [Vertex; 3] = [
//   Vertex {
//     position: [1.0, -1.0, 0.0, 1.0],
//     color: [1.0, 0.0, 0.0, 1.0],
//   },
//   Vertex {
//     position: [-1.0, -1.0, 0.0, 1.0],
//     color: [0.0, 1.0, 0.0, 1.0],
//   },
//   Vertex {
//     position: [0.0, 1.0, 0.0, 1.0],
//     color: [0.0, 0.0, 1.0, 1.0],
//   },
// ];

// const INDICES: [u32; 3] = [0, 1, 2]; // Clockwise winding order

// const SHADER_SOURCE: &str = "
// struct Uniform {
//     mvp: mat4x4<f32>,
// };

// @group(0) @binding(0)
// var<uniform> ubo: Uniform;

// struct VertexInput {
//     @location(0) position: vec4<f32>,
//     @location(1) color: vec4<f32>,
// };
// struct VertexOutput {
//     @builtin(position) position: vec4<f32>,
//     @location(0) color: vec4<f32>,
// };

// @vertex
// fn vertex_main(vert: VertexInput) -> VertexOutput {
//     var out: VertexOutput;
//     out.color = vert.color;
//     out.position = ubo.mvp * vert.position;
//     return out;
// };

// @fragment
// fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
//     return vec4<f32>(in.color);
// }
// ";

// lib.rs
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
  position: [f32; 3],
  // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
  _padding: u32,
  color: [f32; 3],
  // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
  _padding2: u32,
}

struct Instance {
  position: cgmath::Vector3<f32>,
  rotation: cgmath::Quaternion<f32>,
}

impl Instance {
  fn to_raw(&self) -> InstanceRaw {
    let model =
      cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
    InstanceRaw {
      model: model.into(),
      // NEW!
      normal: cgmath::Matrix3::from(self.rotation).into(),
    }
  }
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
  model: [[f32; 4]; 4],
  normal: [[f32; 3]; 3],
}

impl model::Vertex for InstanceRaw {
  fn desc() -> wgpu::VertexBufferLayout<'static> {
    use std::mem;
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
      // We need to switch from using a step mode of Vertex to Instance
      // This means that our shaders will only change to use the next
      // instance when the shader starts processing a new instance
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
          // be using 2, 3, and 4 for Vertex. We'll start at slot 5 to not conflict with them later
          shader_location: 5,
          format: wgpu::VertexFormat::Float32x4,
        },
        // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
        // for each vec4. We don't have to do this in code, though.
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
          shader_location: 6,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
          shader_location: 7,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
          shader_location: 8,
          format: wgpu::VertexFormat::Float32x4,
        },
        // NEW!
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
          shader_location: 9,
          format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
          shader_location: 10,
          format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
          shader_location: 11,
          format: wgpu::VertexFormat::Float32x3,
        },
      ],
    }
  }
}

fn create_render_pipeline(
  device: &wgpu::Device,
  layout: &wgpu::PipelineLayout,
  color_format: wgpu::TextureFormat,
  _depth_format: Option<wgpu::TextureFormat>,
  _vertex_layouts: &[wgpu::VertexBufferLayout],
  shader: wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
  let shader = device.create_shader_module(shader);
  device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(&layout),
    vertex: wgpu::VertexState {
      module: &shader,
      entry_point: "vs_main",                                      // 1.
      buffers: &[model::ModelVertex::desc(), InstanceRaw::desc()], // 2.
      compilation_options: wgpu::PipelineCompilationOptions::default(),
    },
    fragment: Some(wgpu::FragmentState {
      // 3.
      module: &shader,
      entry_point: "fs_main",
      targets: &[Some(wgpu::ColorTargetState {
        // 4.
        format: color_format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
      })],
      compilation_options: wgpu::PipelineCompilationOptions::default(),
    }),
    primitive: wgpu::PrimitiveState {
      topology: wgpu::PrimitiveTopology::TriangleList, // 1.
      strip_index_format: None,
      front_face: wgpu::FrontFace::Ccw, // 2.
      cull_mode: Some(wgpu::Face::Back),
      // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
      polygon_mode: wgpu::PolygonMode::Fill,
      // Requires Features::DEPTH_CLIP_CONTROL
      unclipped_depth: false,
      // Requires Features::CONSERVATIVE_RASTERIZATION
      conservative: false,
    },
    depth_stencil: Some(wgpu::DepthStencilState {
      format: texture::Texture::DEPTH_FORMAT,
      depth_write_enabled: true,
      depth_compare: wgpu::CompareFunction::Less,
      stencil: wgpu::StencilState::default(),
      bias: wgpu::DepthBiasState::default(),
    }), // 1.
    multisample: wgpu::MultisampleState {
      count: 1,                         // 2.
      mask: !0,                         // 3.
      alpha_to_coverage_enabled: false, // 4.
    },
    multiview: None, // 5.
    cache: None,     // 6.
  })
}
