use wgpu::util::DeviceExt;

use crate::transform::Transform;

pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub(crate) uniform_buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: ultraviolet::Mat4,
}

impl Camera {
    pub(crate) fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
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
            }
        )
    }

    /// Updates the uniform and copies it to the bind group
    /// Call after updating the camera's transform
    pub fn update_uniform(&mut self, window: &crate::window::Window) {
        let camera_uniform = CameraUniform {
            view_proj: ultraviolet::projection::perspective_wgpu_dx(self.fov.to_radians(), self.aspect, 0.0, self.far),
        };

        window.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    fn view_matrix(&self) -> ultraviolet::Mat4 {
        ultraviolet::Mat4::look_at(
            self.transform.position,
            self.transform.position + self.transform.forward(),
            self.transform.up(),
        )
    }

    pub fn new(transform: Transform, fov: f32, aspect: f32, near: f32, far: f32, window: &crate::window::Window) -> Self {
        let camera_uniform = CameraUniform {
            // Start with an identity matrix
            view_proj: ultraviolet::Mat4::identity(),
        };

        let uniform_buffer = window.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Uniform Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = Self::get_bind_group_layout(&window.device);

        let bind_group = window.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Camera Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }
                ],
            }
        );
        
        Self {
            transform,
            fov,
            aspect,
            near,
            far,
            uniform_buffer,
            bind_group,
        }
    }
}