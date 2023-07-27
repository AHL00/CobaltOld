#![allow(dead_code)]
#![allow(non_camel_case_types)]
// Internal crate mods
pub(crate) mod core;

// Exported mods
pub extern crate nalgebra_glm as maths;

pub mod app;
pub use app::*;
pub mod ecs;
pub use ecs::*;
pub mod transform;
pub use transform::*;
pub mod renderer;
pub mod resources;
