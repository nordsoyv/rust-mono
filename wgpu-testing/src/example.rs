use winit::window::Window;


use zerocopy::{AsBytes};
use winit::event::WindowEvent;

// main.rs
#[repr(C)]
#[derive(Copy, Clone, Debug,AsBytes)]
struct Vertex {
  position: [f32; 3],
  color: [f32; 3],
}

// main.rs
impl Vertex {
  fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
    use std::mem;
    wgpu::VertexBufferDescriptor {
      stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::InputStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttributeDescriptor {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float3,
        },
        wgpu::VertexAttributeDescriptor {
          offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float3,
        },
      ]
    }
  }
}

// main.rs
const VERTICES: &[Vertex] = &[
  Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
  Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
  Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
  Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
  Vertex { position: [0.44147372, 0.2347359, 0.0],color: [0.5, 0.0, 0.5] }, // E
];

const INDICES: &[u16] = &[
  0, 1, 4,
  1, 2, 4,
  2, 3, 4,
];
const INDICES_LINE: &[u16] = &[
  0, 1, 2, 3, 4, 0
];



pub struct App {
  surface: wgpu::Surface,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  sc_desc: wgpu::SwapChainDescriptor,
  swap_chain: wgpu::SwapChain,
  render_pipeline: wgpu::RenderPipeline,
  line_render_pipeline: wgpu::RenderPipeline,
  num_tri_indices: u32,
  num_line_indices: u32,
  vertex_buffer : wgpu::Buffer,
  index_buffer : wgpu::Buffer,
  line_index_buffer : wgpu::Buffer,
  size: winit::dpi::PhysicalSize<u32>,
  red: f64,
}

impl App {
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
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
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
      present_mode: wgpu::PresentMode::Mailbox,
    };
    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let vs_src = include_str!("shaders/shader.vert");
    let fs_src = include_str!("shaders/shader.frag");
    let fs_line_src = include_str!("shaders/line.frag");

    let vs_spirv = glsl_to_spirv::compile(vs_src, glsl_to_spirv::ShaderType::Vertex).unwrap();
    let fs_spirv = glsl_to_spirv::compile(fs_src, glsl_to_spirv::ShaderType::Fragment).unwrap();
    let fs_line_spirv = glsl_to_spirv::compile(fs_line_src, glsl_to_spirv::ShaderType::Fragment).unwrap();

    let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
    let fs_data = wgpu::read_spirv(fs_spirv).unwrap();
    let fs_line_data = wgpu::read_spirv(fs_line_spirv).unwrap();

    let vs_module = device.create_shader_module(&vs_data);
    let fs_module = device.create_shader_module(&fs_data);
    let fs_line_module = device.create_shader_module(&fs_line_data);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[],
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
        vertex_buffers: &[Vertex::desc()],
      },
      sample_count: 1, // 5.
      sample_mask: !0, // 6.
      alpha_to_coverage_enabled: false, // 7.
    });
    let line_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      layout: &render_pipeline_layout,
      vertex_stage: wgpu::ProgrammableStageDescriptor {
        module: &vs_module,
        entry_point: "main", // 1.
      },
      fragment_stage: Some(wgpu::ProgrammableStageDescriptor { // 2.
        module: &fs_line_module,
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
      primitive_topology: wgpu::PrimitiveTopology::LineStrip, // 1.
      depth_stencil_state: None, // 2.
      vertex_state: wgpu::VertexStateDescriptor {
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[Vertex::desc()],
      },
      sample_count: 1, // 5.
      sample_mask: !0, // 6.
      alpha_to_coverage_enabled: false, // 7.
    });


    let vertex_buffer  = device.create_buffer_with_data(VERTICES.as_bytes(), wgpu::BufferUsage::VERTEX );
    let index_buffer = device.create_buffer_with_data(INDICES.as_bytes(), wgpu::BufferUsage::INDEX);
    let line_index_buffer = device.create_buffer_with_data(INDICES_LINE.as_bytes(), wgpu::BufferUsage::INDEX);

    Self {
      surface,
      adapter,
      device,
      queue,
      sc_desc,
      swap_chain,
      size,
      vertex_buffer,
      index_buffer,
      line_index_buffer,
      render_pipeline,
      line_render_pipeline,
      num_tri_indices: INDICES.len() as u32,
      num_line_indices: INDICES_LINE.len() as u32,
      red: 0.1,
    }
  }

  pub  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    self.size = new_size;
    self.sc_desc.width = new_size.width;
    self.sc_desc.height = new_size.height;
    self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
  }

  pub fn input(&mut self, event: &WindowEvent) -> bool {
    match event {
      WindowEvent::CursorMoved { position, .. } => {
        self.red = (position.x + position.y) / 1000.0;
        true
      }
      _ => {
        false
      }
    }
  }

  pub fn update(&mut self) {}

  pub fn render(&mut self) {
    let frame = self.swap_chain.get_next_texture().unwrap();
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: None
    });
    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[
          wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color {
              r: self.red,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            },
          }
        ],
        depth_stencil_attachment: None,
      });
      render_pass.set_pipeline(&self.render_pipeline); // 2.
      render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0,0);
      render_pass.set_index_buffer(&self.index_buffer,0,0);
//      render_pass.draw(0..self.num_vertices, 0..1); // 3.
      render_pass.draw_indexed(0..self.num_tri_indices, 0, 0..1);

      render_pass.set_pipeline(&self.line_render_pipeline); // 2.
      render_pass.set_index_buffer(&self.line_index_buffer,0,0);
      render_pass.draw_indexed(0..self.num_line_indices, 0, 0..1);


    }

    self.queue.submit(&[
      encoder.finish()
    ]);
  }
}
