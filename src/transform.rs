


pub struct Transform {
    pub position: ultraviolet::Vec3,
    pub rotation: ultraviolet::Rotor3,
    pub scale: ultraviolet::Vec3,
}

impl Transform {
    // TODO: Make more efficient by cacheing the result
    pub(crate) fn view_matrix(&self) -> ultraviolet::Mat4 {
        ultraviolet::Mat4::look_at(
            self.position,
            self.position + self.rotation * ultraviolet::Vec3::unit_z(),
            self.rotation * ultraviolet::Vec3::unit_y(),
        )
    }

    pub fn new(position: ultraviolet::Vec3, rotation: ultraviolet::Rotor3, scale: ultraviolet::Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}