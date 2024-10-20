use wgpu::{util::DeviceExt, ShaderStages};

use crate::{
  camera::Camera,
  gpu::{Gpu, UniformBinding},
  model::{self, DrawModel},
  resource, UiState,
};
use cgmath::prelude::*;

pub struct Scene {
  pub camera: Camera,
  camera_uniform_binding: UniformBinding,
  render_pipeline: wgpu::RenderPipeline,
  light_render_pipeline: wgpu::RenderPipeline,
  obj_model: model::Model,
  instances: Vec<Instance>,
  instance_buffer: wgpu::Buffer,
  diffuse_bind_group: wgpu::BindGroup,
  light_uniform_binding: UniformBinding,
  light_uniform: LightUniform,
  space_between: f32,
  num_instances_per_row: u32,
}

impl<'window> Scene {
  pub async fn new(gpu: &Gpu<'window>, width: u32, height: u32, ui_state: UiState) -> Self {
    let diffuse_bytes = include_bytes!("happy-tree.png");
    let diffuse_texture = gpu
      .create_texture_from_bytes(diffuse_bytes, "happy_tree.png")
      .unwrap();
    let texture_bind_group_layout =
      gpu
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
    let diffuse_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
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

    let camera = Camera::new(
      (0.0, 5.0, 10.0),
      cgmath::Deg(-90.0),
      cgmath::Deg(-20.0),
      width,
      height,
    );
    let camera_uniform_binding = gpu.create_uniform_binding(
      ShaderStages::VERTEX_FRAGMENT,
      bytemuck::cast_slice(&[camera.camera_uniform]),
    );

    let light_uniform = LightUniform {
      position: [2.0, 2.0, 2.0],
      _padding: 0,
      color: [1.0, 1.0, 1.0],
      _padding2: 0,
    };
    let light_uniform_binding = gpu.create_uniform_binding(
      ShaderStages::VERTEX_FRAGMENT,
      bytemuck::cast_slice(&[light_uniform]),
    );
    let (instances, instance_buffer) = create_instances_and_buffer(
      &gpu.device,
      ui_state.num_instances_per_row,
      ui_state.space_between,
    );

    let render_pipeline = {
      let render_pipeline_layout = gpu.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
          &texture_bind_group_layout,
          &camera_uniform_binding.bind_group_layout,
          &light_uniform_binding.bind_group_layout,
        ],
        push_constant_ranges: &[],
      });
      let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Normal Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
      };
      gpu.create_render_pipeline(&render_pipeline_layout, shader, Some("Render pipeline"))
    };

    let light_render_pipeline = {
      let layout = gpu.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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
      gpu.create_render_pipeline(&layout, shader, Some("Light pipeline"))
    };

    let obj_model = resource::load_model(
      "cube.obj",
      &gpu.device,
      &gpu.queue,
      &texture_bind_group_layout,
    )
    .await
    .unwrap();

    Self {
      render_pipeline,
      light_render_pipeline,
      camera,
      camera_uniform_binding,
      obj_model,
      instances,
      instance_buffer,
      diffuse_bind_group,
      light_uniform_binding,
      light_uniform,
      num_instances_per_row: ui_state.num_instances_per_row,
      space_between: ui_state.space_between,
    }
  }

  pub fn render<'rpass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'rpass>) {
    render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
    render_pass.set_bind_group(1, &self.camera_uniform_binding.bind_group, &[]);
    render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

    use crate::model::DrawLight;
    render_pass.set_pipeline(&self.light_render_pipeline);
    render_pass.draw_light_model(
      &self.obj_model,
      &self.camera_uniform_binding.bind_group,
      &self.light_uniform_binding.bind_group,
    );

    render_pass.set_pipeline(&self.render_pipeline);

    render_pass.draw_model_instanced(
      &self.obj_model,
      0..self.instances.len() as u32,
      &self.camera_uniform_binding.bind_group,
      &self.light_uniform_binding.bind_group,
    );
  }

  pub fn update_instaces(&mut self, queue: &wgpu::Queue, new_instances: &Vec<Instance>) {
    let instance_data = new_instances
      .iter()
      .map(Instance::to_raw)
      .collect::<Vec<_>>();
    queue.write_buffer(
      &self.instance_buffer,
      0,
      bytemuck::cast_slice(&instance_data),
    );
  }

  pub fn update(
    &mut self,
    queue: &wgpu::Queue,
    aspect_ratio: f32,
    ui_state: &UiState,
    delta_time: f32,
  ) {
    self.camera.update_camera(delta_time);
    queue.write_buffer(
      &self.camera_uniform_binding.buffer,
      0,
      bytemuck::cast_slice(&[self.camera.camera_uniform]),
    );

    if self.space_between != ui_state.space_between || self.num_instances_per_row != ui_state.num_instances_per_row{
      let instances = create_instances(ui_state.num_instances_per_row, ui_state.space_between);
      self.update_instaces(queue, &instances);
    }

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

pub(crate) fn create_instances(num_instances_per_row: u32, space_between: f32) -> Vec<Instance> {
  (0..num_instances_per_row)
    .flat_map(|z| {
      (0..num_instances_per_row).map(move |x| {
        let x = space_between * (x as f32 - num_instances_per_row as f32 / 2.0);
        let z = space_between * (z as f32 - num_instances_per_row as f32 / 2.0);

        let position = cgmath::Vector3 { x, y: 0.0, z };

        let rotation = if position.is_zero() {
          cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
        } else {
          cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
        };

        Instance { position, rotation }
      })
    })
    .collect::<Vec<_>>()
}

pub(crate) fn create_instances_and_buffer(
  device: &wgpu::Device,
  num_instances_per_row: u32,
  space_between: f32,
) -> (Vec<Instance>, wgpu::Buffer) {
  let instances = create_instances(num_instances_per_row, space_between);
  let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
  let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Instance Buffer"),
    contents: bytemuck::cast_slice(&instance_data),
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
  });
  (instances, instance_buffer)
}

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

pub struct Instance {
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
pub struct InstanceRaw {
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
