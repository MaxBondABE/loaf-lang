use crate::lang::runtime::naive::{Coordinate, StateId};

pub mod render2d;

// TODO genericize
pub type Delta = Vec<(Coordinate, StateId)>;

pub trait Output {
    fn output_tick(&mut self, delta: Delta);
}
