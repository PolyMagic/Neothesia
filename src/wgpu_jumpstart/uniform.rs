use wgpu::util::DeviceExt;

use bytemuck::Pod;

pub struct Uniform<U>
where
    U: Pod,
{
    pub data: U,
    buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}
impl<U> Uniform<U>
where
    U: Pod,
{
    pub fn new(device: &wgpu::Device, data: U, visibility: wgpu::ShaderStages) -> Self {
        let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<U>() as u64),
                },
                count: None,
            }],
        };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&bind_group_layout_descriptor);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        Self {
            data,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
    pub fn update(&self, command_encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device) {
        let staging_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[self.data]),
            usage: wgpu::BufferUsages::COPY_SRC,
        });

        command_encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.buffer,
            0,
            std::mem::size_of::<U>() as wgpu::BufferAddress,
        );
    }
}
