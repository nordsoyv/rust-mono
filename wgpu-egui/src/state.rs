use std::sync::Arc;

use cgmath::prelude::*;
use wgpu::util::DeviceExt;
use winit::{ window::Window};
use crate::{
  camera::{Camera, CameraController, Projection},
  model::{self, DrawModel, Vertex},
  resource, texture,
};

pub struct State<'a> {
  surface: wgpu::Surface<'a>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  render_pipeline: wgpu::RenderPipeline,
  //window: &'a Window,
  diffuse_bind_group: wgpu::BindGroup,
  _diffuse_texture: texture::Texture,
  camera: Camera,
  projection: Projection,
  camera_bind_group: wgpu::BindGroup,
  camera_buffer: wgpu::Buffer,
  camera_uniform: CameraUniform,
  camera_controller: CameraController,
  instances: Vec<Instance>,
  instance_buffer: wgpu::Buffer,
  depth_texture: texture::Texture,
  obj_model: model::Model,
  light_buffer: wgpu::Buffer,
  light_uniform: LightUniform,
  light_bind_group: wgpu::BindGroup,
  light_render_pipeline: wgpu::RenderPipeline,
  _mouse_pressed: bool,
  last_render_time: std::time::Instant,
}

impl<'a> State<'a> {
  // Creating some of the wgpu types requires async code
  pub async fn new(window: Arc<Window>) -> State<'a> {
    let size = window.inner_size();

    // The instance is a handle to our GPU
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      ..Default::default()
    });

    let surface = instance.create_surface(Arc::clone(&window)).unwrap();

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
          required_features: wgpu::Features::POLYGON_MODE_LINE,
          // WebGL doesn't support all of wgpu's features, so if
          // we're building for the web, we'll have to disable some.
          required_limits: wgpu::Limits::default(),
          label: None,
          memory_hints: Default::default(),
        },
        None, // Trace path
      )
      .await
      .unwrap();
    let surface_caps = surface.get_capabilities(&adapter);

    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result in all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps
      .formats
      .iter()
      .find(|f| f.is_srgb())
      .copied()
      .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

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

    let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth texture");

    let camera = Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
    let projection = Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
    let camera_controller = CameraController::new(4.0, 2.0);

    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera, &projection);
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Camera buffer"),
      contents: bytemuck::cast_slice(&[camera_uniform]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_bind_group_layout =
      device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        }],
        label: Some("camera_bind_group_layout"),
      });
    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &camera_bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
      label: Some("camera_bind_group"),
    });

    let light_uniform = LightUniform {
      position: [2.0, 2.0, 2.0],
      _padding: 0,
      color: [1.0, 1.0, 1.0],
      _padding2: 0,
    };

    // We'll want to update our lights position, so we use COPY_DST
    let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Light VB"),
      contents: bytemuck::cast_slice(&[light_uniform]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let light_bind_group_layout =
      device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        }],
        label: None,
      });

    let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &light_bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: light_buffer.as_entire_binding(),
      }],
      label: None,
    });

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
    let _shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[
        &texture_bind_group_layout,
        &camera_bind_group_layout,
        &light_bind_group_layout,
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
        config.format,
        Some(texture::Texture::DEPTH_FORMAT),
        &[model::ModelVertex::desc(), InstanceRaw::desc()],
        shader,
      )
    };
    // lib.rs
    let light_render_pipeline = {
      let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Light Pipeline Layout"),
        bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
        push_constant_ranges: &[],
      });
      let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Light Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
      };
      create_render_pipeline(
        &device,
        &layout,
        config.format,
        Some(texture::Texture::DEPTH_FORMAT),
        &[model::ModelVertex::desc()],
        shader,
      )
    };

    let obj_model = resource::load_model("cube.obj", &device, &queue, &texture_bind_group_layout)
      .await
      .unwrap();

    Self {
      //  window,
      surface,
      device,
      queue,
      config,
      size,
      render_pipeline,
      diffuse_bind_group,
      _diffuse_texture: diffuse_texture,
      camera,
      projection,
      camera_uniform,
      camera_buffer,
      camera_bind_group,
      camera_controller,
      instances,
      instance_buffer,
      depth_texture,
      obj_model,
      light_buffer,
      light_uniform,
      light_bind_group,
      light_render_pipeline,
      _mouse_pressed: false,
      last_render_time: std::time::Instant::now(),
    }
  }

  // pub fn window(&self) -> &Window {
  //   &self.window
  // }

  pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
    self.size
  }
  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    dbg!("resize");
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
      self.depth_texture =
        texture::Texture::create_depth_texture(&self.device, &self.config, "depth texture");
      self.projection.resize(new_size.width, new_size.height);
    }
  }
  /*
    fn input(&mut self, event: &WindowEvent) -> bool {
      match event {
        WindowEvent::KeyboardInput {
          event:
            KeyEvent {
              physical_key: PhysicalKey::Code(key),
              state,
              ..
            },
          ..
        } => self.camera_controller.process_keyboard(*key, *state),
        WindowEvent::MouseWheel { delta, .. } => {
          self.camera_controller.process_scroll(delta);
          true
        }
        WindowEvent::MouseInput {
          button: MouseButton::Left,
          state,
          ..
        } => {
          self._mouse_pressed = *state == ElementState::Pressed;
          true
        }
        _ => false,
      }
    }
  */
  pub fn update(&mut self) {
    let dt = self.get_dt();
    self.camera_controller.update_camera(&mut self.camera, dt);
    self
      .camera_uniform
      .update_view_proj(&self.camera, &self.projection);
    self.queue.write_buffer(
      &self.camera_buffer,
      0,
      bytemuck::cast_slice(&[self.camera_uniform]),
    );

    let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
    self.light_uniform.position = (cgmath::Quaternion::from_axis_angle(
      (0.0, 1.0, 0.0).into(),
      cgmath::Deg(60.0 * dt.as_secs_f32()),
    ) * old_position)
      .into();
    self.queue.write_buffer(
      &self.light_buffer,
      0,
      bytemuck::cast_slice(&[self.light_uniform]),
    );
  }

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });
    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[
          // This is what @location(0) in the fragment shader targets
          Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
              }),
              store: wgpu::StoreOp::Store,
            },
          }),
        ],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: &self.depth_texture.view,
          depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: wgpu::StoreOp::Store,
          }),
          stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
      });

      render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
      render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
      render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

      use crate::model::DrawLight; // NEW!
      render_pass.set_pipeline(&self.light_render_pipeline); // NEW!
      render_pass.draw_light_model(
        &self.obj_model,
        &self.camera_bind_group,
        &self.light_bind_group,
      ); // NEW!

      render_pass.set_pipeline(&self.render_pipeline); // 2.

      render_pass.draw_model_instanced(
        &self.obj_model,
        0..self.instances.len() as u32,
        &self.camera_bind_group,
        &self.light_bind_group,
      );
      // 3.
    }

    // submit will accept anything that implements IntoIter
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }

  pub(crate) fn get_dt(&mut self) -> std::time::Duration {
    let now = std::time::Instant::now();
    let dt = now - self.last_render_time;
    self.last_render_time = now;
    dt
  }
}

// NEW!
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
  model: [[f32; 4]; 4],
  normal: [[f32; 3]; 3],
}

const NUM_INSTANCES_PER_ROW: u32 = 10;
const SPACE_BETWEEN: f32 = 3.0;
// const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
//   NUM_INSTANCES_PER_ROW as f32 * 0.5,
//   0.0,
//   NUM_INSTANCES_PER_ROW as f32 * 0.5,
// );

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

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
  view_position: [f32; 4],
  // We can't use cgmath with bytemuck directly, so we'll have
  // to convert the Matrix4 into a 4x4 f32 array
  view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
  fn new() -> Self {
    use cgmath::SquareMatrix;
    Self {
      view_position: [0.0; 4],
      view_proj: cgmath::Matrix4::identity().into(),
    }
  }

  fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
    self.view_position = camera.position.to_homogeneous().into();
    self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
  }
}

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
