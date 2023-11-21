use ultraviolet::Vec2;

use crate::{physics::Physics, transform::Transform};

pub mod rigidbody;

pub(crate) trait Simulatable2D {
    fn simulate(&mut self, delta: f32, transform: &mut Transform, constants: &PhysicsConstants);
}

pub struct PhysicsConstants {
    pub gravity: Vec2,
}

pub struct Physics2D {
    // Sim variables
    pub time_step: f32,

    // World variables
    pub constants: PhysicsConstants,

    last_sim: std::time::Instant,
}

impl Physics2D {
    pub fn new() -> Self {
        Self {
            time_step: 1.0 / 60.0,
            constants: PhysicsConstants {
                gravity: Vec2::new(0.0, -9.81),
            },
            last_sim: std::time::Instant::now(),
        }
    }
}

impl Physics for Physics2D {
    fn simulate(&mut self, world: &mut hecs::World) {
        let now = std::time::Instant::now();
        let delta = now - self.last_sim;
        self.last_sim = now;

        let delta = delta.as_secs_f32();

        for (entity, (transform, rigidbody)) in world
            .query::<(&mut crate::Transform, &mut rigidbody::Rigidbody2D)>()
            .iter()
        {
            rigidbody.simulate(delta, transform, &self.constants);
        }
    }
}
