use ultraviolet::{Mat4, Vec4};
use wgpu::util::DeviceExt;

use crate::{transform::Transform, window::Window};

pub struct Camera {
    pub transform: Transform,
    pub projection: Projection,

    pub(crate) uniform_buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,

    cached_projection_matrix: Option<ultraviolet::Mat4>,
    cached_view_matrix: Option<ultraviolet::Mat4>,
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
            view_proj: proj * view,
        };

        window.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    fn projection_matrix(&mut self) -> ultraviolet::Mat4 {
        let mat = match self.projection {
            Projection::Perspective {
                fov,
                aspect,
                near,
                far,
            } => ultraviolet::projection::perspective_reversed_z_wgpu_dx_gl(
                fov.to_radians(),
                aspect,
                near,
                far,
            ),
            Projection::Orthographic {
                aspect,
                height,
                near,
                far,
            } => {
                let pos = self.transform.position();

                let left = -aspect * height / 2.0 + pos.x;
                let right = aspect * height / 2.0 + pos.x;
                let bottom = -height / 2.0 + pos.y;
                let top = height / 2.0 + pos.y;

                ultraviolet::projection::orthographic_wgpu_dx(left, right, bottom, top, near, far)
            }
        };

        self.cached_projection_matrix = Some(mat);

        mat
    }

    fn view_matrix(&mut self) -> ultraviolet::Mat4 {
        let pos = self.transform.position();
        let up = self.transform.up();
        let forward = self.transform.forward();

        // Code copied from cgmath::Matrix4::look_to_rh
        // let f = dir.normalize();
        // let s = f.cross(up).normalize();
        // let u = s.cross(f);

        // #[cfg_attr(rustfmt, rustfmt_skip)]
        // Matrix4::new(
        //     s.x.clone(), u.x.clone(), -f.x.clone(), S::zero(),
        //     s.y.clone(), u.y.clone(), -f.y.clone(), S::zero(),
        //     s.z.clone(), u.z.clone(), -f.z.clone(), S::zero(),
        //     -eye.dot(s), -eye.dot(u), eye.dot(f), S::one(),
        // )

        let f = forward.normalized();
        let s = f.cross(up).normalized();
        let u = s.cross(f);

        let mat = Mat4::new(
            Vec4::new(s.x, u.x, -f.x, 0.0),
            Vec4::new(s.y, u.y, -f.y, 0.0),
            Vec4::new(s.z, u.z, -f.z, 0.0),
            Vec4::new(-pos.dot(s), -pos.dot(u), pos.dot(f), 1.0),
        );

        self.cached_view_matrix = Some(mat);

        mat
    }

    /// Converts a point in world space to screen space
    /// x and y are the coordinates on the screen
    /// z is the depth value in the range of -1.0 to 1.0
    pub fn world_to_screen(&self, window: &Window, point: &ultraviolet::Vec3) -> ultraviolet::Vec3 {
        if self.cached_projection_matrix.is_none() || self.cached_view_matrix.is_none() {
            log::error!("world_to_screen() called before camera matrices were ever cached, returning (0, 0, 0)");
            return ultraviolet::Vec3::zero();
        }

        let view_proj = self.cached_projection_matrix.unwrap() * self.cached_view_matrix.unwrap();

        let clip = view_proj * ultraviolet::Vec4::new(point.x, point.y, point.z, 1.0);

        let ndc = clip / clip.w;

        let win_size = window.winit_win.inner_size();

        ultraviolet::Vec3::new(
            (ndc.x + 1.0) / 2.0 * win_size.width as f32,
            (1.0 - ndc.y) / 2.0 * win_size.height as f32,
            ndc.z,
        )
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
            cached_projection_matrix: None,
            cached_view_matrix: None,
        }
    }
}
