

pub trait Physics {
    /// Run as fast as possible.
    /// The check for the time step is done in the physics system.
    fn simulate(&mut self, world: &mut hecs::World);
}