use std::{time::Instant, collections::VecDeque};

use ultraviolet::Vec2;

use crate::transform::Transform;

use super::{Simulatable2D, PhysicsConstants};

pub enum ForceMode2D {
    /// Applies a force continuously until forces are reset
    Continuous,
    /// Applies a force for one simulation step
    Impulse,
    /// The instant is the time until which the force will be applied
    Timed(Instant),
}

struct Force2D {
    pub force: Vec2,
    pub mode: ForceMode2D,
}

pub struct Rigidbody2D {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub friction: f32,

    forces: Vec<Force2D>,

    pub constrain_x: bool,
    pub constrain_y: bool,
    pub constrain_rot_z: bool,
}

impl Rigidbody2D {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::zero(),
            acceleration: Vec2::zero(),
            mass: 1.0,
            friction: 0.0,

            forces: Vec::new(),

            constrain_x: false,
            constrain_y: false,
            constrain_rot_z: false,
        }
    }

    pub fn add_force(&mut self, force: Vec2, mode: ForceMode2D) {
        self.forces.push(Force2D {
            force,
            mode,
        });
    }
}

impl Simulatable2D for Rigidbody2D {
    fn simulate(&mut self, delta: f32, transform: &mut Transform, constants: &PhysicsConstants) {
        // Reset acceleration
        self.acceleration = Vec2::zero();

        // Apply forces
        for (i, force) in self.forces.iter().enumerate() {
            match force.mode {
                ForceMode2D::Continuous => {
                    self.acceleration += force.force / self.mass;
                },
                ForceMode2D::Impulse => {
                    self.velocity += force.force / self.mass;
                },
                ForceMode2D::Timed(_) => {
                    self.acceleration += force.force / self.mass;
                },
            }
        }

        // Remove expired forces
        self.forces.retain(|force| {
            match force.mode {
                ForceMode2D::Continuous => true,
                ForceMode2D::Impulse => false,
                ForceMode2D::Timed(instant) => Instant::now() < instant,
            }
        });

        // Apply gravity
        self.acceleration += constants.gravity;

        // Apply friction
        self.acceleration -= self.velocity * self.friction;

        // Apply acceleration
        self.velocity += self.acceleration * delta;

        // Apply velocity
        self.velocity *= 1.0 - self.friction;

        // Apply velocity to transform
        transform.position_mut().x += self.velocity.x * delta;
        transform.position_mut().y += self.velocity.y * delta;
    }
}