use nalgebra_glm as glm;

/// 2D Transform
#[derive(Clone, Copy, Debug)]
pub struct Transform2D {
    pub position: glm::Vec2,
    pub rotation: f32,
    pub scale: glm::Vec2,
}

impl Transform2D {
    pub fn new() -> Transform2D {
        Transform2D {
            position: glm::vec2(0.0, 0.0),
            rotation: 0.0,
            scale: glm::vec2(1.0, 1.0),
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.position.x += x;
        self.position.y += y;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.scale.x *= x;
        self.scale.y *= y;
    }

    pub fn get_model_matrix(&self) -> glm::Mat4 {
        let mut model = glm::identity();
        model = glm::translate(&model, &glm::vec3(self.position.x, self.position.y, 0.0));
        model = glm::rotate(&model, self.rotation, &glm::vec3(0.0, 0.0, 1.0));
        model = glm::scale(&model, &glm::vec3(self.scale.x, self.scale.y, 1.0));
        model
    }
}

