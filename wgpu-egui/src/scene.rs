use wgpu::{util::DeviceExt, ShaderStages};

use crate::{
  camera::{Camera, CameraController, CameraUniform, Projection}, model::{self, DrawModel, Vertex}, resource, texture, UniformBinding
};
use cgmath::prelude::*;

pub struct Scene {
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
    let light_uniform_binding = UniformBinding::new(
      device,
      ShaderStages::VERTEX_FRAGMENT,
      bytemuck::cast_slice(&[light_uniform]),
    );

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

  }
}

const NUM_INSTANCES_PER_ROW: u32 = 10;
const SPACE_BETWEEN: f32 = 3.0;

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
