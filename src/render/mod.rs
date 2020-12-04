use crate::lang::runtime::datatypes::coords::Coordinate;
use crate::lang::runtime::StateId;

pub mod render2d;

// TODO genericize
pub type Delta = Vec<(Coordinate, StateId)>;

pub trait Output {
    fn output_tick(&mut self, delta: Delta);
}
