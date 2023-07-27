use nalgebra_glm as glm;

/// 2D Transform
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    position: glm::Vec2,
    rotation: f32,
    scale: glm::Vec2,
    model_matrix: glm::Mat4,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: glm::vec2(0.0, 0.0),
            rotation: 0.0,
            scale: glm::vec2(1.0, 1.0),
            model_matrix: glm::identity(),
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.position.x += x;
        self.position.y += y;
        self.model_matrix = glm::translate(&self.model_matrix, &glm::vec3(x, y, 0.0));
    }

    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
        self.model_matrix = glm::rotate(&self.model_matrix, angle, &glm::vec3(0.0, 0.0, 1.0));
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.scale.x *= x;
        self.scale.y *= y;
        self.model_matrix = glm::scale(&self.model_matrix, &glm::vec3(x, y, 1.0));
    }

    pub fn get_model_matrix(&self) -> &glm::Mat4 {
        &self.model_matrix
    }

    pub fn get_position(&self) -> &glm::Vec2 {
        &self.position
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_scale(&self) -> &glm::Vec2 {
        &self.scale
    }
}
