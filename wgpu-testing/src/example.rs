use bytemuck::{Pod, Zeroable};
use wgpu::{BufferDescriptor, BufferUsage};
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};
use winit::event::WindowEvent::KeyboardInput;
use winit::window::Window;
use zerocopy::AsBytes;
use cgmath::prelude::*;

use crate::texture::Texture;
use crate::vertex::Vertex;
use crate::vertex::VertexWithTex;
use crate::wgpu_utils::create_shader_module;

const VERTICES: &[VertexWithTex] = &[
  VertexWithTex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 1.0 - 0.99240386] }, // A
  VertexWithTex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 1.0 - 0.56958646] }, // B
  VertexWithTex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 1.0 - 0.050602943] }, // C
  VertexWithTex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 1.0 - 0.15267089] }, // D
  VertexWithTex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 1.0 - 0.7347359] }, // E
];

const INDICES: &[u16] = &[
  0, 1, 4,
  1, 2, 4,
  2, 3, 4,
];
const INDICES_LINE: &[u16] = &[
  0, 1, 2, 3, 4, 0
];

const NUM_INSTANCES_PER_ROW: u32 = 10;
const NUM_INSTANCES: u32 = NUM_INSTANCES_PER_ROW * NUM_INSTANCES_PER_ROW;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(NUM_INSTANCES_PER_ROW as f32 * 0.5, 0.0, NUM_INSTANCES_PER_ROW as f32 * 0.5);




struct Camera {
  eye: cgmath::Point3<f32>,
  target: cgmath::Point3<f32>,
  up: cgmath::Vector3<f32>,
  aspect: f32,
  fovy: f32,
  znear: f32,
  zfar: f32,
}

impl Camera {
  fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
    // 1.
    let view = cgmath::Matrix4::look_at(self.eye, self.target, self.up);
    // 2.
    let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

    // 3.
    return proj * view;
  }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.0,
  0.0, 0.0, 0.5, 1.0,
);

#[repr(C)] // We need this for Rust to store our data correctly for the shaders
#[derive(Debug, Copy, Clone)] // This is so we can store this in a buffer
struct Uniforms {
  view_proj: cgmath::Matrix4<f32>,
  model: cgmath::Matrix4<f32>,
}

unsafe impl Pod for Uniforms {}

unsafe impl Zeroable for Uniforms {}

impl Uniforms {
  fn new() -> Self {
    use cgmath::SquareMatrix;
    Self {
      view_proj: cgmath::Matrix4::identity(),
      model: cgmath::Matrix4::identity(),
    }
  }

  fn update_view_proj(&mut self, camera: &Camera) {
    self.view_proj = OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix();
  }
}


struct Instance {
  position: cgmath::Vector3<f32>,
  rotation: cgmath::Quaternion<f32>,
}

impl Instance {
  fn to_matrix(&self) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)
  }
}

struct CameraController {
  speed: f32,
  is_up_pressed: bool,
  is_down_pressed: bool,
  is_forward_pressed: bool,
  is_backward_pressed: bool,
  is_left_pressed: bool,
  is_right_pressed: bool,
}

impl CameraController {
  fn new(speed: f32) -> Self {
    Self {
      speed,
      is_up_pressed: false,
      is_down_pressed: false,
      is_forward_pressed: false,
      is_backward_pressed: false,
      is_left_pressed: false,
      is_right_pressed: false,
    }
  }

  fn process_events(&mut self, event: &WindowEvent) -> bool {
    match event {
      WindowEvent::KeyboardInput {
        input: winit::event::KeyboardInput {
          state,
          virtual_keycode: Some(keycode),
          ..
        },
        ..
      } => {
        let is_pressed = *state == ElementState::Pressed;
        match keycode {
          VirtualKeyCode::Space => {
            self.is_up_pressed = is_pressed;
            true
          }
          VirtualKeyCode::LShift => {
            self.is_down_pressed = is_pressed;
            true
          }
          VirtualKeyCode::W | VirtualKeyCode::Up => {
            self.is_forward_pressed = is_pressed;
            true
          }
          VirtualKeyCode::A | VirtualKeyCode::Left => {
            self.is_left_pressed = is_pressed;
            true
          }
          VirtualKeyCode::S | VirtualKeyCode::Down => {
            self.is_backward_pressed = is_pressed;
            true
          }
          VirtualKeyCode::D | VirtualKeyCode::Right => {
            self.is_right_pressed = is_pressed;
            true
          }
          _ => false,
        }
      }
      _ => false,
    }
  }
  fn update_camera(&self, camera: &mut Camera) {
    use cgmath::InnerSpace;
    let forward = camera.target - camera.eye;
    let forward_norm = forward.normalize();
    let forward_mag = forward.magnitude();

    // Prevents glitching when camera gets too close to the
    // center of the scene.
    if self.is_forward_pressed && forward_mag > self.speed {
      camera.eye += forward_norm * self.speed;
    }
    if self.is_backward_pressed {
      camera.eye -= forward_norm * self.speed;
    }

    let right = forward_norm.cross(camera.up);

    // Redo radius calc in case the up/ down is pressed.
    let forward = camera.target - camera.eye;
    let forward_mag = forward.magnitude();

    if self.is_right_pressed {
      // Rescale the distance between the target and eye so
      // that it doesn't change. The eye therefore still
      // lies on the circle made by the target and eye.
      camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
    }
    if self.is_left_pressed {
      camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
    }
  }
}


pub struct Example {
  surface: wgpu::Surface,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  sc_desc: wgpu::SwapChainDescriptor,
  swap_chain: wgpu::SwapChain,
  render_pipeline: wgpu::RenderPipeline,
  num_tri_indices: u32,
  num_line_indices: u32,
  vertex_buffer: wgpu::Buffer,
  index_buffer: wgpu::Buffer,
  line_index_buffer: wgpu::Buffer,
  size: winit::dpi::PhysicalSize<u32>,

  diffuse_texture: Texture,
  happy_tree_bind_group: wgpu::BindGroup,
  unhappy_tree_bind_group: wgpu::BindGroup,
  happy: bool,
  camera: Camera,
  camera_controller: CameraController,
  uniforms: Uniforms,
  uniform_buffer: wgpu::Buffer,
  uniform_bind_group: wgpu::BindGroup,
  instances : Vec<Instance>
}

impl Example {
  pub async fn new(window: &Window) -> Self {
    let size = window.inner_size();

    let surface = wgpu::Surface::create(window);

    let adapter = wgpu::Adapter::request(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        compatible_surface: Some(&surface),
      },
      wgpu::BackendBit::PRIMARY,
    ).await.unwrap();
    let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
      extensions: wgpu::Extensions {
        anisotropic_filtering: false,
      },
      limits: wgpu::Limits::default(),
    }).await;

    let sc_desc = wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
      format: wgpu::TextureFormat::Bgra8UnormSrgb,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
    };
    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // load texture
    let (happy_tree_texture, cmd_buffer) = Texture::from_bytes(&device, include_bytes!("happy-tree.png")).unwrap();
    queue.submit(&[cmd_buffer]);
    let (unhappy_tree_texture, cmd_buffer) = Texture::from_bytes(&device, include_bytes!("unhappy-tree.png")).unwrap();
    queue.submit(&[cmd_buffer]);
    let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      bindings: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::SampledTexture {
            multisampled: false,
            dimension: wgpu::TextureViewDimension::D2,
            component_type: wgpu::TextureComponentType::Uint,
          },
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStage::FRAGMENT,
          ty: wgpu::BindingType::Sampler { comparison: false },
        },
      ],
      label: None,
    });


    let happy_tree_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &texture_bind_group_layout,
      bindings: &[
        wgpu::Binding {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&happy_tree_texture.view),
        },
        wgpu::Binding {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&happy_tree_texture.sampler),
        }
      ],
      label: None,
    });
    let unhappy_tree_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &texture_bind_group_layout,
      bindings: &[
        wgpu::Binding {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&unhappy_tree_texture.view),
        },
        wgpu::Binding {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&unhappy_tree_texture.sampler),
        }
      ],
      label: None,
    });

    let camera = Camera {
      // position the camera one unit up and 2 units back
      eye: (0.0, 1.0, 2.0).into(),
      // have it look at the origin
      target: (0.0, 0.0, 0.0).into(),
      // which way is "up"
      up: cgmath::Vector3::unit_y(),
      aspect: sc_desc.width as f32 / sc_desc.height as f32,
      fovy: 45.0,
      znear: 0.1,
      zfar: 100.0,
    };

    let camera_controller = CameraController::new(0.2);

    let mut uniforms = Uniforms::new();
    uniforms.update_view_proj(&camera);

//    let uniform_buffer = device.create_buffer_with_data(&uniforms.to_bytes(), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);
    let uniform_buffer = device.create_buffer_with_data(bytemuck::bytes_of(&uniforms), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);
    // The COPY_DST part will be important later
//      .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
//      .fill_from_slice(&[uniforms]);


    let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      bindings: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStage::VERTEX, // 1.
          ty: wgpu::BindingType::UniformBuffer {
            dynamic: false, // 2.
          },
        }
      ],
      label: None,
    });

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &uniform_bind_group_layout,
      bindings: &[
        wgpu::Binding {
          binding: 0,
          resource: wgpu::BindingResource::Buffer {
            buffer: &uniform_buffer,
            // FYI: you can share a single buffer between bindings.
            range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
          },
        }
      ],
      label: None,
    });


    let vs_module = create_shader_module(&device, include_str!("shaders/shader.vert"), glsl_to_spirv::ShaderType::Vertex);
    let fs_module = create_shader_module(&device, include_str!("shaders/shader.frag"), glsl_to_spirv::ShaderType::Fragment);
    let fs_line_module = create_shader_module(&device, include_str!("shaders/line.frag"), glsl_to_spirv::ShaderType::Fragment);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
    });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      layout: &render_pipeline_layout,
      vertex_stage: wgpu::ProgrammableStageDescriptor {
        module: &vs_module,
        entry_point: "main", // 1.
      },
      fragment_stage: Some(wgpu::ProgrammableStageDescriptor { // 2.
        module: &fs_module,
        entry_point: "main",
      }),
      rasterization_state: Some(wgpu::RasterizationStateDescriptor {
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: wgpu::CullMode::Back,
        depth_bias: 0,
        depth_bias_slope_scale: 0.0,
        depth_bias_clamp: 0.0,
      }),
      color_states: &[
        wgpu::ColorStateDescriptor {
          format: sc_desc.format,
          color_blend: wgpu::BlendDescriptor::REPLACE,
          alpha_blend: wgpu::BlendDescriptor::REPLACE,
          write_mask: wgpu::ColorWrite::ALL,
        },
      ],
      primitive_topology: wgpu::PrimitiveTopology::TriangleList, // 1.
      depth_stencil_state: None, // 2.
      vertex_state: wgpu::VertexStateDescriptor {
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[VertexWithTex::desc()],
      },
      sample_count: 1, // 5.
      sample_mask: !0, // 6.
      alpha_to_coverage_enabled: false, // 7.
    });


    let vertex_buffer = device.create_buffer_with_data(VERTICES.as_bytes(), wgpu::BufferUsage::VERTEX);
    let index_buffer = device.create_buffer_with_data(INDICES.as_bytes(), wgpu::BufferUsage::INDEX);
    let line_index_buffer = device.create_buffer_with_data(INDICES_LINE.as_bytes(), wgpu::BufferUsage::INDEX);

    let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
      (0..NUM_INSTANCES_PER_ROW).map(move |x| {
        let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - INSTANCE_DISPLACEMENT;
        let rotation = if position.is_zero() {
          // this is needed so an object at (0, 0, 0) won't get scaled to zero
          // as Quaternions can effect scale if they're not create correctly
          cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
        } else {
          cgmath::Quaternion::from_axis_angle(position.clone().normalize(), cgmath::Deg(45.0))
        };

        Instance {
          position, rotation,
        }
      })
    }).collect();

    Self {
      surface,
      adapter,
      device,
      queue,
      sc_desc,
      swap_chain,
      size,
      diffuse_texture: happy_tree_texture,
      vertex_buffer,
      index_buffer,
      line_index_buffer,
      render_pipeline,
      num_tri_indices: INDICES.len() as u32,
      num_line_indices: INDICES_LINE.len() as u32,
      happy_tree_bind_group,
      unhappy_tree_bind_group,
      happy: true,
      camera,
      uniforms,
      uniform_buffer,
      uniform_bind_group,
      camera_controller,
      instances,
    }
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    self.size = new_size;
    self.sc_desc.width = new_size.width;
    self.sc_desc.height = new_size.height;
    self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
  }

  pub fn input(&mut self, event: &WindowEvent) -> bool {
    let result = self.camera_controller.process_events(event);
    if result {
      return result;
    }
    match event {
      WindowEvent::KeyboardInput {
        input,
        ..
      } => {
        match input {
          winit::event::KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Space),
            ..
          } => {
            self.happy = !self.happy;
            true
          }
          _ => {
            false
          }
        }
      }
      _ => {
        false
      }
    }
  }

  pub fn update(&mut self) {
    self.camera_controller.update_camera(&mut self.camera);
    self.uniforms.update_view_proj(&self.camera);

    // Copy operation's are performed on the gpu, so we'll need
    // a CommandEncoder for that
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("update encoder"),
    });

    let staging_buffer = self.device.create_buffer_with_data(
      bytemuck::cast_slice(&[self.uniforms]),
      wgpu::BufferUsage::COPY_SRC,
    );

    encoder.copy_buffer_to_buffer(&staging_buffer, 0, &self.uniform_buffer, 0, std::mem::size_of::<Uniforms>() as wgpu::BufferAddress);

    // We need to remember to submit our CommandEncoder's output
    // otherwise we won't see any change.
    self.queue.submit(&[encoder.finish()]);
  }

  pub fn render(&mut self) {
    let frame = self.swap_chain.get_next_texture().unwrap();
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: None
    });
    {
      encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[
          wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            },
          }
        ],
        depth_stencil_attachment: None,
      });
    }
    for instance in &self.instances {
      // 1.
      self.uniforms.model = instance.to_matrix();
      let staging_buffer = self.device.create_buffer_with_data(
        bytemuck::cast_slice(&[self.uniforms]),
        wgpu::BufferUsage::COPY_SRC,
      );
      encoder.copy_buffer_to_buffer(&staging_buffer, 0, &self.uniform_buffer, 0, std::mem::size_of::<Uniforms>() as wgpu::BufferAddress);

      // 2.
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[
          wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Load, // 3.
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            },
          }
        ],
        depth_stencil_attachment: None,
      });

      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.set_bind_group(0, &self.happy_tree_bind_group, &[]);
      render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
      render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
      render_pass.set_index_buffer(&self.index_buffer, 0, 0);
      render_pass.draw_indexed(0..self.num_tri_indices, 0, 0..1);
    }
    self.queue.submit(&[
      encoder.finish()
    ]);
  }
}
