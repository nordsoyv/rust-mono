use winit::event::WindowEvent;
use winit::window::Window;
use zerocopy::AsBytes;

use crate::wgpu_utils::create_shader_module;
use crate::vertex::{Vertex};

const MARGIN: f32  = 0.1;

const VERTICES: &[Vertex] = &[
  Vertex { position: [MARGIN, MARGIN, 0.0] }, //A
  Vertex { position: [1.0- MARGIN, MARGIN, 0.0] }, //B
  Vertex { position: [MARGIN, 1.0- MARGIN, 0.0] }, //C
  Vertex { position: [1.0- MARGIN, 1.0- MARGIN, 0.0] }, //D

  Vertex { position: [MARGIN, 0.0, 0.0] }, //E
  Vertex { position: [1.0- MARGIN, 0.0, 0.0] }, //F

  Vertex { position: [0.0, MARGIN, 0.0] }, //G
  Vertex { position: [1.0, MARGIN, 0.0] }, //H

  Vertex { position: [0.0, 1.0- MARGIN, 0.0] }, //I
  Vertex { position: [1.0, 1.0- MARGIN, 0.0] }, //J

  Vertex { position: [MARGIN, 1.0, 0.0] }, //K
  Vertex { position: [1.0- MARGIN, 1.0, 0.0] }, //L
];

const INDICES_FILL: &[u16] = &[
  0, 1, 2,
  1, 3, 2,
];
const INDICES_LINE: &[u16] = &[
  4,0,0,6,
  5,1,1,7,
  8,2,2,10,
  9,3,3,11
];


pub struct Grid {
  surface: wgpu::Surface,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  sc_desc: wgpu::SwapChainDescriptor,
  swap_chain: wgpu::SwapChain,
  fill_render_pipeline: wgpu::RenderPipeline,
  line_render_pipeline: wgpu::RenderPipeline,
  num_tri_indices: u32,
  num_line_indices: u32,
  vertex_buffer: wgpu::Buffer,
  fill_index_buffer: wgpu::Buffer,
  line_index_buffer: wgpu::Buffer,
  size: winit::dpi::PhysicalSize<u32>,
}

impl Grid {
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

    let vs_module = create_shader_module(&device, include_str!("shaders/shader.vert"), glsl_to_spirv::ShaderType::Vertex);
    let fs_module = create_shader_module(&device, include_str!("shaders/shader.frag"), glsl_to_spirv::ShaderType::Fragment);
    let fs_line_module = create_shader_module(&device, include_str!("shaders/line.frag"), glsl_to_spirv::ShaderType::Fragment);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[],
    });
    let fill_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
      primitive_topology: wgpu::PrimitiveTopology::LineList, // 1.
      depth_stencil_state: None, // 2.
      vertex_state: wgpu::VertexStateDescriptor {
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[Vertex::desc()],
      },
      sample_count: 1, // 5.
      sample_mask: !0, // 6.
      alpha_to_coverage_enabled: false, // 7.
    });


    let vertex_buffer = device.create_buffer_with_data(VERTICES.as_bytes(), wgpu::BufferUsage::VERTEX);
    let fill_index_buffer = device.create_buffer_with_data(INDICES_FILL.as_bytes(), wgpu::BufferUsage::INDEX);
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
      fill_index_buffer,
      line_index_buffer,
      fill_render_pipeline,
      line_render_pipeline,
      num_tri_indices: INDICES_FILL.len() as u32,
      num_line_indices: INDICES_LINE.len() as u32,
    }
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    self.size = new_size;
    self.sc_desc.width = new_size.width;
    self.sc_desc.height = new_size.height;
    self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
  }

  pub fn input(&mut self, event: &WindowEvent) -> bool {
    match event {
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
              r: 1.0,
              g: 1.0,
              b: 1.0,
              a: 1.0,
            },
          }
        ],
        depth_stencil_attachment: None,
      });
      render_pass.set_pipeline(&self.fill_render_pipeline); // 2.
      render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
      render_pass.set_index_buffer(&self.fill_index_buffer, 0, 0);
      render_pass.draw_indexed(0..self.num_tri_indices, 0, 0..1);

      render_pass.set_pipeline(&self.line_render_pipeline); // 2.
      render_pass.set_index_buffer(&self.line_index_buffer, 0, 0);
      render_pass.draw_indexed(0..self.num_line_indices, 0, 0..1);
    }

    self.queue.submit(&[
      encoder.finish()
    ]);
  }
}
