use std::sync::Arc;
use vulkano::VulkanLibrary;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::{
  StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::{
  DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo, DescriptorType,
};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::layout::{PipelineDescriptorSetLayoutCreateInfo, PipelineLayoutCreateInfo};
use vulkano::pipeline::{
  ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo,
};
use vulkano::sync::{self, GpuFuture};
mod cs {
  vulkano_shaders::shader! {
      ty: "compute",
      src: r"
            #version 460

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

            layout(set = 0, binding = 0) buffer Data {
                uint data[];
            } buf;

            void main() {
                uint idx = gl_GlobalInvocationID.x;
                buf.data[idx] *= 12;
            }
        ",
  }
}

fn main() {
  let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
  let instance = Instance::new(
    library,
    InstanceCreateInfo {
      flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
      ..Default::default()
    },
  )
  .expect("failed to create instance");
  let physical_device = instance
    .enumerate_physical_devices()
    .expect("could not enumerate devices")
    .next()
    .expect("no devices available");

  let queue_family_index = physical_device
    .queue_family_properties()
    .iter()
    .position(|queue_family_properties| {
      queue_family_properties
        .queue_flags
        .contains(QueueFlags::COMPUTE)
    })
    .expect("couldn't find a compute queue family") as u32;

  let (device, mut queues) = Device::new(
    physical_device,
    DeviceCreateInfo {
      // here we pass the desired queue family to use by index
      queue_create_infos: vec![QueueCreateInfo {
        queue_family_index,
        ..Default::default()
      }],
      enabled_extensions: DeviceExtensions {
        khr_storage_buffer_storage_class: true,
        ..DeviceExtensions::empty()
      },
      ..Default::default()
    },
  )
  .expect("failed to create device");

  let queue = queues.next().unwrap();
  let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

  let data_iter = 0..65536u32;
  let data_buffer = Buffer::from_iter(
    memory_allocator.clone(),
    BufferCreateInfo {
      usage: BufferUsage::STORAGE_BUFFER,
      ..Default::default()
    },
    AllocationCreateInfo {
      memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
      ..Default::default()
    },
    data_iter,
  )
  .expect("failed to create buffer");

  let shader = cs::load(device.clone()).expect("failed to create shader module");
  let cs = shader.entry_point("main").unwrap();
  let stage = PipelineShaderStageCreateInfo::new(cs);

  let set_layout = DescriptorSetLayout::new(
    device.clone(),
    DescriptorSetLayoutCreateInfo {
      bindings: [(0, DescriptorType::StorageBuffer)].into_iter().collect(),
      ..Default::default()
    },
  )
  .unwrap();

  let layout = PipelineLayout::new(
    device.clone(),
    PipelineLayoutCreateInfo {
      set_layouts: vec![set_layout.clone()],
      ..Default::default()
    },
  )
  .unwrap();

  // let layout = PipelineLayout::new(
  //   device.clone(),
  //   PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
  //     .into_pipeline_layout_create_info(device.clone())
  //     .unwrap(),
  // )
  // .unwrap();

  // let compute_pipeline = ComputePipeline::new(
  //   device.clone(),
  //   None,
  //   ComputePipelineCreateInfo::stage_layout(stage, layout),
  // )
  // .expect("failed to create compute pipeline");

  // let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
  //   device.clone(),
  //   Default::default(),
  // ));
  // let pipeline_layout = compute_pipeline.layout();
  // let descriptor_set_layouts = pipeline_layout.set_layouts();
  // let descriptor_set_layout_index = 0;
  // let descriptor_set_layout = descriptor_set_layouts
  //   .get(descriptor_set_layout_index)
  //   .unwrap();
  // let descriptor_set = DescriptorSet::new(
  //   descriptor_set_allocator.clone(),
  //   descriptor_set_layout.clone(),
  //   [WriteDescriptorSet::buffer(0, data_buffer.clone())], // 0 is the binding
  //   [],
  // )
  // .unwrap();
  //
  // let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
  //   device.clone(),
  //   StandardCommandBufferAllocatorCreateInfo::default(),
  // ));
  //
  // let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
  //   command_buffer_allocator.clone(),
  //   queue.queue_family_index(),
  //   CommandBufferUsage::OneTimeSubmit,
  // )
  // .unwrap();
  //
  // let work_group_counts = [1024, 1, 1];

  // unsafe {
  //   command_buffer_builder
  //     .bind_pipeline_compute(compute_pipeline.clone())
  //     .unwrap()
  //     .bind_descriptor_sets(
  //       PipelineBindPoint::Compute,
  //       compute_pipeline.layout().clone(),
  //       descriptor_set_layout_index as u32,
  //       descriptor_set,
  //     )
  //     .unwrap()
  //     .dispatch(work_group_counts)
  //     .unwrap();
  // }

  // let command_buffer = command_buffer_builder.build().unwrap();
  // let future = sync::now(device.clone())
  //   .then_execute(queue.clone(), command_buffer)
  //   .unwrap()
  //   .then_signal_fence_and_flush()
  //   .unwrap();
  // future.wait(None).unwrap(); // None is an optional timeout
  // let content = data_buffer.read().unwrap();
  // for (n, val) in content.iter().enumerate() {
  //   assert_eq!(*val, n as u32 * 12);
  // }
  // let src_content = source.read().unwrap();
  // let destination_content = destination.read().unwrap();
  // assert_eq!(&*src_content, &*destination_content);

  println!("Hello, world!");
}
