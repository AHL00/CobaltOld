


pub struct Transform {
    pub position: ultraviolet::Vec3,
    pub rotation: ultraviolet::Rotor3,
    pub scale: ultraviolet::Vec3,
}

impl Transform {

    // TODO: Make more efficient by cacheing the result
    pub(crate) fn model_matrix(&self) -> ultraviolet::Mat4 {
        unimplemented!("Transform::model_matrix()");
        // ultraviolet::Mat4::from_translation(self.position) * ultraviolet::Mat4::from_rotation(self.rotation) * ultraviolet::Mat4::from_scale(self.scale)
    }

    pub fn new(position: ultraviolet::Vec3, rotation: ultraviolet::Rotor3, scale: ultraviolet::Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
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