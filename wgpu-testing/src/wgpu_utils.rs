use wgpu::ShaderModule;

pub fn create_shader_module(device: &wgpu::Device, src: &str, shader_type: glsl_to_spirv::ShaderType) -> ShaderModule {
  let spirv = glsl_to_spirv::compile(src, shader_type).unwrap();
  let data = wgpu::read_spirv(spirv).unwrap();
  device.create_shader_module(&data)
}