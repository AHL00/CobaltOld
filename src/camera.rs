use wgpu::util::DeviceExt;

use crate::transform::Transform;

pub struct Camera {
    pub transform: Transform,
    pub projection: Projection,

    pub(crate) uniform_buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
}

pub enum Projection {
    Perspective {
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    },
    Orthographic {
        aspect: f32,
        height: f32,
        near: f32,
        far: f32,
    },
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: ultraviolet::Mat4,
}

const OPENGL_TO_WGPU: &ultraviolet::Mat4 = &ultraviolet::Mat4 {
    cols: [
        ultraviolet::Vec4 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        ultraviolet::Vec4 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        },
        ultraviolet::Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.5,
            w: 0.5,
        },
        ultraviolet::Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
    ],
};

impl Camera {
    pub(crate) fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        })
    }

    /// Updates the uniform and copies it to the bind group
    /// Call after updating the camera's transform
    pub fn update_uniform(&mut self, window: &crate::window::Window) {
        let view = self.view_matrix();
        let proj = self.projection_matrix();

        let camera_uniform = CameraUniform {
            view_proj: *OPENGL_TO_WGPU * proj * view,
        };

        window.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    fn projection_matrix(&self) -> ultraviolet::Mat4 {
        match self.projection {
            Projection::Perspective {
                fov,
                aspect,
                near,
                far,
            } => ultraviolet::projection::perspective_reversed_z_wgpu_dx_gl(
                fov.to_radians(), aspect, near, far,
            ),
            Projection::Orthographic {
                aspect,
                height,
                near,
                far,
            } => {

            let pos = self.transform.position;

            let left = -aspect * height / 2.0 + pos.x;
            let right = aspect * height / 2.0 + pos.x;
            let bottom = -height / 2.0 + pos.y;
            let top = height / 2.0 + pos.y;
                
            ultraviolet::projection::orthographic_wgpu_dx(
                left, right, bottom, top, near, far,
            )
        
        },
        }
    }

    fn view_matrix(&self) -> ultraviolet::Mat4 {
        let (pos_x, pos_y, pos_z) = self.transform.position.into();
        let (up_x, up_y, up_z) = self.transform.up().into();
        let (forward_x, forward_y, forward_z) = self.transform.forward().into();

        // let cg_mat = cgmath::Matrix4::look_at_rh(
        //     cgmath::Point3::new(pos_x, pos_y, pos_z),
        //     cgmath::Point3::new(0.0, 0.0, 0.0),
        //     cgmath::Vector3::new(up_x, up_y, up_z),
        // );

        let cg_mat = cgmath::Matrix4::look_to_rh(
            cgmath::Point3::new(pos_x, pos_y, pos_z),
            cgmath::Vector3::new(forward_x, forward_y, forward_z),
            cgmath::Vector3::new(up_x, up_y, up_z),
        );

        let mat = Self::cgmath_to_ultraviolet_mat4(cg_mat);

        mat
    }

    fn cgmath_to_ultraviolet_mat4(mat: cgmath::Matrix4<f32>) -> ultraviolet::Mat4 {
        ultraviolet::Mat4 {
            cols: [
                ultraviolet::Vec4 {
                    x: mat.x.x,
                    y: mat.x.y,
                    z: mat.x.z,
                    w: mat.x.w,
                },
                ultraviolet::Vec4 {
                    x: mat.y.x,
                    y: mat.y.y,
                    z: mat.y.z,
                    w: mat.y.w,
                },
                ultraviolet::Vec4 {
                    x: mat.z.x,
                    y: mat.z.y,
                    z: mat.z.z,
                    w: mat.z.w,
                },
                ultraviolet::Vec4 {
                    x: mat.w.x,
                    y: mat.w.y,
                    z: mat.w.z,
                    w: mat.w.w,
                },
            ],
        }
    }

    pub fn new(
        transform: Transform,
        projection: Projection,
        window: &crate::window::Window,
    ) -> Self {
        let camera_uniform = CameraUniform {
            // Start with an identity matrix
            view_proj: ultraviolet::Mat4::identity(),
        };

        let uniform_buffer = window
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Uniform Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group_layout = Self::get_bind_group_layout(&window.device);

        let bind_group = window.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            transform,
            projection,
            uniform_buffer,
            bind_group,
        }
    }
}
