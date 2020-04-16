pub struct Texture {
  pub texture: wgpu::Texture,
  pub view: wgpu::TextureView,
  pub sampler: wgpu::Sampler,
}


impl Texture {
  // 1.
  pub fn from_bytes(device: &wgpu::Device, bytes: &[u8]) -> Result<(Self, wgpu::CommandBuffer), failure::Error> {
    let png = std::io::Cursor::new(bytes);
    let decoder = png::Decoder::new(png);
    let (info, mut reader) = decoder.read_info().expect("can read info");
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).expect("can read png frame");
//    buf
    Self::from_image(device, buf, info.width, info.height)
  }

  pub fn from_image(device: &wgpu::Device, img: Vec<u8>, width: u32, height: u32) -> Result<(Self, wgpu::CommandBuffer), failure::Error> {
//    let rgba = img.as_rgba8().unwrap();
//    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
      width,
      height,
      depth: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: None,
      size,
      array_layer_count: 1,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    let mut diffuse_buffer = device.create_buffer_with_data(&img, wgpu::BufferUsage::COPY_SRC);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: None
    });

    encoder.copy_buffer_to_texture(
      wgpu::BufferCopyView {
        buffer: &diffuse_buffer,
        offset: 0,
//        row_pitch: 4 * dimensions.0, // the width of the texture in bytes
//        image_height: dimensions.1,
        bytes_per_row: 4 * width,
        rows_per_image: height,
      },
      wgpu::TextureCopyView {
        texture: &texture,
        mip_level: 0,
        array_layer: 0,
        origin: wgpu::Origin3d::ZERO,
      },
      size,
    );



    let cmd_buffer = encoder.finish(); // 2.

    let view = texture.create_default_view();

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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
    Ok((Self { texture, view, sampler }, cmd_buffer))
  }
}