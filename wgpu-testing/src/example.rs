use wgpu::{BufferDescriptor, BufferUsage};
use winit::event::WindowEvent;
use winit::window::Window;
use zerocopy::AsBytes;

use crate::wgpu_utils::create_shader_module;
use crate::vertex::VertexWithTex;
use crate::vertex::Vertex;

const VERTICES: &[VertexWithTex] = &[
  VertexWithTex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 1.0-0.99240386],}, // A
  VertexWithTex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 1.0-0.56958646] }, // B
  VertexWithTex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 1.0-0.050602943] }, // C
  VertexWithTex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 1.0-0.15267089]}, // D
  VertexWithTex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 1.0-0.7347359]}, // E
];

const INDICES: &[u16] = &[
  0, 1, 4,
  1, 2, 4,
  2, 3, 4,
];
const INDICES_LINE: &[u16] = &[
  0, 1, 2, 3, 4, 0
];


pub struct Example {
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
  vertex_buffer: wgpu::Buffer,
  index_buffer: wgpu::Buffer,
  line_index_buffer: wgpu::Buffer,
  size: winit::dpi::PhysicalSize<u32>,
  diffuse_texture: wgpu::Texture,
  diffuse_texture_view: wgpu::TextureView,
  diffuse_sampler: wgpu::Sampler,
  diffuse_bind_group: wgpu::BindGroup,
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
      present_mode: wgpu::PresentMode::Mailbox,
    };
    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let paths: [&'static [u8]; 1] = [
      &include_bytes!("happy-tree.png")[..],
    ];

    let (mut image_width, mut image_height) = (0, 0);
    let faces = paths
      .iter()
      .map(|png| {
        let png = std::io::Cursor::new(png);
        let decoder = png::Decoder::new(png);
        let (info, mut reader) = decoder.read_info().expect("can read info");
        image_width = info.width;
        image_height = info.height;
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).expect("can read png frame");
        buf
      })
      .collect::<Vec<_>>();



    let tex_size = wgpu::Extent3d {
      width: image_width,
      height: image_height,
      depth: 1,
    };
    let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
      // All textures are stored as 3d, we represent our 2d texture
      // by setting depth to 1.
      label: None,
      size: wgpu::Extent3d {
        width: image_width,
        height: image_height,
        depth: 1,
      },
      // You can store multiple textures of the same size in one
      // Texture object
      array_layer_count: 1,
      mip_level_count: 1, // We'll talk about this a little later
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      // SAMPLED tells wgpu that we want to use this texture in shaders
      // COPY_DST means that we want to copy data to this texture
      usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,

    });

    let mut diffuse_buffer = device
      .create_buffer_with_data(&faces[0], BufferUsage::COPY_SRC);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: None
    });

    encoder.copy_buffer_to_texture(
      wgpu::BufferCopyView {
        buffer: &diffuse_buffer,
        offset: 0,
//        row_pitch: 4 * dimensions.0, // the width of the texture in bytes
//        image_height: dimensions.1,
        bytes_per_row: 4 * image_width,
        rows_per_image: image_height,
      },
      wgpu::TextureCopyView {
        texture: &diffuse_texture,
        mip_level: 0,
        array_layer: 0,
        origin: wgpu::Origin3d::ZERO,
      },
      tex_size,
    );

    queue.submit(&[encoder.finish()]);


    // We don't need to configure the texture view much, so let's
// let wgpu define it.
    let diffuse_texture_view = diffuse_texture.create_default_view();

    let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      lod_min_clamp: -100.0,
      lod_max_clamp: 100.0,
      compare: wgpu::CompareFunction::Always,
    });


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


    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &texture_bind_group_layout,
      bindings: &[
        wgpu::Binding {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
        },
        wgpu::Binding {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
        }
      ],
      label: None,
    });


    let vs_module = create_shader_module(&device, include_str!("shaders/shader.vert"), glsl_to_spirv::ShaderType::Vertex);
    let fs_module = create_shader_module(&device, include_str!("shaders/shader.frag"), glsl_to_spirv::ShaderType::Fragment);
    let fs_line_module = create_shader_module(&device, include_str!("shaders/line.frag"), glsl_to_spirv::ShaderType::Fragment);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      bind_group_layouts: &[&texture_bind_group_layout],
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
        vertex_buffers: &[VertexWithTex::desc()],
      },
      sample_count: 1, // 5.
      sample_mask: !0, // 6.
      alpha_to_coverage_enabled: false, // 7.
    });


    let vertex_buffer = device.create_buffer_with_data(VERTICES.as_bytes(), wgpu::BufferUsage::VERTEX);
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
      diffuse_texture,
      diffuse_texture_view,
      diffuse_sampler,
      vertex_buffer,
      index_buffer,
      line_index_buffer,
      render_pipeline,
      line_render_pipeline,
      num_tri_indices: INDICES.len() as u32,
      num_line_indices: INDICES_LINE.len() as u32,
      diffuse_bind_group,
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
      render_pass.set_pipeline(&self.render_pipeline); // 2.
      render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
      render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
      render_pass.set_index_buffer(&self.index_buffer, 0, 0);
      render_pass.draw_indexed(0..self.num_tri_indices, 0, 0..1);

//      render_pass.set_pipeline(&self.line_render_pipeline); // 2.
//      render_pass.set_index_buffer(&self.line_index_buffer, 0, 0);
//      render_pass.draw_indexed(0..self.num_line_indices, 0, 0..1);
    }

    self.queue.submit(&[
      encoder.finish()
    ]);
  }
}
