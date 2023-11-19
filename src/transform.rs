use wgpu::util::DeviceExt;


pub struct Transform {
    position: ultraviolet::Vec3,
    rotation: ultraviolet::Rotor3,
    scale: ultraviolet::Vec3,
    
    dirty: bool,
    model_matrix: ultraviolet::Mat4,
    // It's fine to let these be None at the start, because the renderer will always call update_uniform before drawing
    pub(crate) bind_group: Option<wgpu::BindGroup>,
    pub(crate) uniform_buffer: Option<wgpu::Buffer>,
}

impl Transform {
    /// Returns a reference to the model matrix.
    /// If the transform is dirty, it recalculates the model matrix.
    pub(crate) fn model_matrix(&mut self) -> &ultraviolet::Mat4 {
        if self.dirty {
            self.recalculate_model_matrix();
            self.dirty = false;
        }

        &self.model_matrix
    }

    /// If dirty, recalculates the matrix, then updates the uniform buffer with the model matrix.
    /// If not dirty, does nothing.
    pub(crate) fn update_uniform(&mut self, window: &crate::window::Window) {
        // TODO: These checks for ergonomics could be removed if transform asked for a reference to device
        // in the new function rather than creating it itself

        if self.dirty {
            self.recalculate_model_matrix();

            if self.uniform_buffer.is_none() {
                self.uniform_buffer = Some(
                    window.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Transform Uniform Buffer"),
                        contents: bytemuck::cast_slice(&[*self.model_matrix()]),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    }),
                );
            }

            window.queue.write_buffer(
                self.uniform_buffer.as_ref().unwrap(),
                0,
                bytemuck::cast_slice(&[self.model_matrix]),
            );
            
            if self.bind_group.is_none() {
                self.bind_group = Some(
                    window.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("Transform Bind Group"),
                        layout: &Self::get_bind_group_layout(&window.device),
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: if let Some(buffer) = &self.uniform_buffer {
                                buffer.as_entire_binding()
                            } else {
                                log::error!("Transform uniform buffer should not be None when updating uniform");
                                panic!();
                            },
                        }],
                    }),
                );
            }
        }
    }

    fn recalculate_model_matrix(&mut self) {
        self.model_matrix = ultraviolet::Mat4::from_translation(self.position)
            * self.rotation.into_matrix().into_homogeneous()
            * ultraviolet::Mat4::from_nonuniform_scale(self.scale);
    }

    pub(crate) fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Transform Bind Group Layout"),
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

    pub fn new(position: ultraviolet::Vec3, rotation: ultraviolet::Vec3, scale: ultraviolet::Vec3) -> Self {
        Self {
            position,
            rotation: ultraviolet::Rotor3::from_euler_angles(rotation.x, rotation.y, rotation.z),
            scale,
            dirty: true,
            model_matrix: ultraviolet::Mat4::identity(),
            bind_group: None,
            uniform_buffer: None,
        }
    }

    pub fn position(&self) -> &ultraviolet::Vec3 {
        &self.position
    }

    pub fn rotation(&self) -> &ultraviolet::Rotor3 {
        &self.rotation
    }

    pub fn scale(&self) -> &ultraviolet::Vec3 {
        &self.scale
    }

    // Returns a mutable reference to the position
    // Sets the transform to dirty, which means the model matrix needs to be recalculated
    pub fn position_mut(&mut self) -> &mut ultraviolet::Vec3 {
        self.dirty = true;
        &mut self.position
    }

    // Returns a mutable reference to the rotation
    // Sets the transform to dirty, which means the model matrix needs to be recalculated
    pub fn rotation_mut(&mut self) -> &mut ultraviolet::Rotor3 {
        self.dirty = true;
        &mut self.rotation
    }

    // Returns a mutable reference to the scale
    // Sets the transform to dirty, which means the model matrix needs to be recalculated
    pub fn scale_mut(&mut self) -> &mut ultraviolet::Vec3 {
        self.dirty = true;
        &mut self.scale
    }

    pub fn forward(&self) -> ultraviolet::Vec3 {
        self.rotation * ultraviolet::Vec3::unit_z()
    }

    pub fn right(&self) -> ultraviolet::Vec3 {
        self.rotation * ultraviolet::Vec3::unit_x()
    }

    pub fn up(&self) -> ultraviolet::Vec3 {
        self.rotation * ultraviolet::Vec3::unit_y()
    }
}