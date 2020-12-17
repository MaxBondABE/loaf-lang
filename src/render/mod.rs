use crate::lang::runtime::datatypes::coords::Coordinate;
use crate::lang::runtime::StateId;

pub mod render2d;

// TODO genericize
pub type Delta = Vec<(Coordinate, StateId)>;

pub trait Output {
    fn output_tick(&mut self, delta: Delta);
}

/// Dummy Output which does nothing
/// This allows us to avoid making Program.output an Option, which would
/// impose a check & penalize the general case when an Output exists
pub struct NullOutput {}
impl Output for NullOutput {
    fn output_tick(&mut self, delta: Delta) {}
}
