use wgpu::{util::DeviceExt, Device, Queue};

pub(crate) struct Uniform<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) uniform_buffer: wgpu::Buffer,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Uniform<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub(crate) fn new(device: &Device, data: &T, binding: u32) -> Self {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &Self::get_bind_group_layout(device),
            entries: &[wgpu::BindGroupEntry {
                binding,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            bind_group,
            uniform_buffer,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    pub(crate) fn update(&self, data: &T, queue: &Queue) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[*data]));
    }
}
