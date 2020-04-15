use zerocopy::AsBytes;

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct VertexWithColor {
  pub position: [f32; 3],
  pub color: [f32; 3],
}

// main.rs
impl VertexWithColor {
  pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
    use std::mem;
    wgpu::VertexBufferDescriptor {
      stride: mem::size_of::<VertexWithColor>() as wgpu::BufferAddress,
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
      ],
    }
  }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct VertexWithTex {
  pub position: [f32; 3],
  pub tex_coords: [f32; 2],
}


impl VertexWithTex {
  pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
    use std::mem;
    wgpu::VertexBufferDescriptor {
      stride: mem::size_of::<VertexWithTex>() as wgpu::BufferAddress,
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
          format: wgpu::VertexFormat::Float2,
        },
      ],
    }
  }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Vertex {
  pub position: [f32; 3],
}

// main.rs
impl Vertex {
  pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
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
      ],
    }
  }
}
