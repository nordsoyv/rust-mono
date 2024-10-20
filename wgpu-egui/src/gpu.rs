use anyhow::*;
use wgpu::ShaderStages;

use crate::{model::{self, Vertex}, scene::InstanceRaw, texture::{self, Texture}};

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

  pub fn create_texture_from_bytes(&self, bytes: &[u8], label: &str) -> Result<Texture> {
    texture::Texture::from_bytes(&self.device, &self.queue, bytes, label)
  }

  pub fn create_uniform_binding(
    &self,
    shader_stages: ShaderStages,
    contents: &[u8],
  ) -> UniformBinding {
    UniformBinding::new(&self.device, shader_stages, contents)
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

  pub(crate) fn create_pipeline_layout(
    &self,
    descriptor: &wgpu::PipelineLayoutDescriptor<'_>,
  ) -> wgpu::PipelineLayout {
    self.device.create_pipeline_layout(descriptor)
  }

  pub(crate) fn create_render_pipeline(
    &self,
    render_pipeline_layout: &wgpu::PipelineLayout,
    shader: wgpu::ShaderModuleDescriptor<'_>,
    label: Option<&str>
  ) -> wgpu::RenderPipeline {
    let shader = self.device.create_shader_module(shader);
    self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label,
      layout: Some(&render_pipeline_layout),
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
          format: self.surface_format,
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
}

pub struct UniformBinding {
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
