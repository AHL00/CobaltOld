use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transform {
    position: ultraviolet::Vec3,
    rotation: ultraviolet::Rotor3,
    scale: ultraviolet::Vec3,

    dirty: bool,
    model_matrix: ultraviolet::Mat4,
}

impl Transform {
    /// Returns a reference to the model matrix.
    /// Does not automatically recalculate the model matrix.
    pub(crate) fn model_matrix(&mut self) -> &ultraviolet::Mat4 {
        &self.model_matrix
    }

    /// Recalculates the model matrix.
    pub(crate) fn recalculate_matrix(&mut self) -> bool {
        if self.dirty {
            self.model_matrix = ultraviolet::Mat4::from_translation(self.position)
                * self.rotation.into_matrix().into_homogeneous()
                * ultraviolet::Mat4::from_nonuniform_scale(self.scale);

            self.dirty = false;
            
            true
        } else {
            false
        }
    }

    pub fn new(
        position: ultraviolet::Vec3,
        rotation: ultraviolet::Vec3,
        scale: ultraviolet::Vec3,
    ) -> Self {
        Self {
            position,
            rotation: ultraviolet::Rotor3::from_euler_angles(rotation.x, rotation.y, rotation.z),
            scale,
            dirty: true,
            model_matrix: ultraviolet::Mat4::identity(),
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
