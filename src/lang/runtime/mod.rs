use std::collections::HashMap;

use crate::render::Delta;

use self::datatypes::coords::Coordinate;

pub mod naive;
pub mod ops;
pub mod datatypes;

// TODO make types generic over StateId
// TODO move to datetypes::states
pub(crate) type StateId = usize;
pub(crate) type StateMap = HashMap<String, StateId>;

pub trait Runtime {
    fn run_tick(&mut self) -> Delta;
    fn run(&mut self, ticks: usize);
    fn tick(&self) -> usize;
    fn get_env(&self) -> HashMap<Coordinate, StateId>;
    fn get_state(&self, coord: Coordinate) -> Option<StateId>;
    fn set_env(&mut self, environment: HashMap<Coordinate, StateId>);
    fn set_cell(&mut self, coord: Coordinate, state: StateId) -> Option<StateId>;
}
