use crate::lang::runtime::naive::Coordinate::{Coordinate1D, Coordinate2D, Coordinate3D};
use std::collections::HashMap;
use crate::lang::parse::blocks::boundary::BoundaryBlock;
use crate::lang::runtime::naive::ops::rules::Rules;
use crate::lang::runtime::naive::ops::neighborhood::Neighborhood;
use crate::lang::parse::blocks::state::{StatesBlock, Attribute};
use std::slice::Iter;
use std::ops::{RangeInclusive};
use std::iter::{Zip, repeat, Repeat};
use crate::lang::runtime::naive::DimensionBounds::*;
use std::mem::{swap, zeroed};

mod ops;

pub(crate) type StateId = usize;
type StateMap = HashMap<String, StateId>;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Coordinate {
    Coordinate1D {x: isize},
    Coordinate2D {x: isize, y: isize},
    Coordinate3D {x: isize, y: isize, z: isize}
}
impl Coordinate {
    pub fn add_x(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { x } => {
                Coordinate1D {x: x + magnitude}
            }
            Coordinate2D { x, y } => {
                Coordinate2D { x: x + magnitude, y}
            }
            Coordinate3D { x, y, z } => {
                Coordinate3D { x: x + magnitude, y, z}
            }
        }
    }
    pub fn add_y(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { .. } => panic!("Illegal coordinate operation: 1D coordinate has no Y value"),
            Coordinate2D { x, y } => {
                Coordinate2D { x, y: y + magnitude }
            }
            Coordinate3D { x, y, z } => {
                Coordinate3D { x, y: y + magnitude, z }
            }
        }
    }
    pub fn add_z(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { .. } => panic!("Illegal coordinate operation: 1D coordinate has no Z value"),
            Coordinate2D { .. }  => panic!("Illegal coordinate operation: 2D coordinate has no Z value"),
            Coordinate3D { x, y, z } => {
                Coordinate3D { x, y, z: z + magnitude }
            }
        }
    }
    pub fn add_all(self, magnitude: isize) -> Vec<Coordinate> {
        match self {
            Coordinate::Coordinate1D { x } => {
                vec!(Coordinate1D {x: x + magnitude})
            }
            Coordinate::Coordinate2D { x, y } => {
                vec!(
                    Coordinate2D {x: x + magnitude, y},
                    Coordinate2D {x, y: y + magnitude}
                )
            }
            Coordinate::Coordinate3D { x, y, z } => {
                vec!(
                    Coordinate3D {x: x + magnitude, y, z},
                    Coordinate3D {x, y: y + magnitude, z},
                    Coordinate3D {x, y, z: z + magnitude},
                )
            }
        }
    }
    pub fn sub_x(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { x } => {
                Coordinate1D {x: x - magnitude}
            }
            Coordinate2D { x, y } => {
                Coordinate2D { x: x - magnitude, y }
            }
            Coordinate3D { x, y, z } => {
                Coordinate3D { x: x - magnitude, y, z }
            }
        }
    }
    pub fn sub_y(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { .. } => panic!("Illegal coordinate operation: 1D coordinate has no Y value"),
            Coordinate2D { x, y } => {
                Coordinate2D { x, y: y - magnitude }
            }
            Coordinate3D { x, y, z } => {
                Coordinate3D { x, y: y - magnitude, z }
            }
        }
    }
    pub fn sub_z(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { .. } => panic!("Illegal coordinate operation: 1D coordinate has no Z value"),
            Coordinate2D { .. } => panic!("Illegal coordinate operation: 2D coordinate has no Z value"),
            Coordinate3D { x, y, z } => {
                Coordinate3D { x, y, z: z - magnitude }
            }
        }
    }
    pub fn sub_all(self, magnitude: isize) -> Vec<Coordinate> {
        match self {
            Coordinate::Coordinate1D { x } => {
                vec!(Coordinate1D {x: x - magnitude})
            }
            Coordinate::Coordinate2D { x, y } => {
                vec!(
                    Coordinate2D {x: x - magnitude, y},
                    Coordinate2D {x, y: y - magnitude}
                )
            }
            Coordinate::Coordinate3D { x, y, z } => {
                vec!(
                    Coordinate3D {x: x - magnitude, y, z},
                    Coordinate3D {x, y: y - magnitude, z},
                    Coordinate3D {x, y, z: z - magnitude},
                )
            }
        }
    }
}

pub struct States {
    num_states: StateId,
    state_map: StateMap,
    default: Option<StateId>
}
impl States {
    pub fn new(states_block: StatesBlock) -> Self {
        let states_block = states_block.into_map();
        let num_states = states_block.iter().count();
        let mut default = None;
        let mut state_map = HashMap::new();
        for (state_id, (name, attributes)) in states_block.into_iter().enumerate() {
            state_map.insert(name, state_id);
            if attributes.iter().find(|a| **a == Attribute::Default).is_some() {
                default = Some(state_id);
            }
        }
        Self {
            num_states,
            state_map,
            default
        }
    }
    pub fn state_map(&self) -> StateMap {
        self.state_map.clone()
    }
    pub fn default_state(&self) -> Option<usize> { self.default }
}

pub type Bound = (isize, isize);
fn within_bound(coord: isize, bound: Bound) -> bool {
    let (low, high) = bound;
    low <= coord && coord <= high
}
fn at_bound(coord: isize, bound: Bound) -> bool {
    let (low, high) = bound;
    low == coord || coord == high
}
fn bound_breadth(bound: Bound) -> isize {
    let (low, high) = bound;
    high - low + 1
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DimensionBounds {
    DimensionBounds1D { x: Bound },
    DimensionBounds2D { x: Bound, y: Bound },
    DimensionBounds3D { x: Bound, y: Bound, z: Bound }
}
impl DimensionBounds {
    pub fn contains(self, coord: Coordinate) -> bool {
        match (coord, self) {
            (Coordinate1D { x }, DimensionBounds1D { x: x_bound }) => within_bound(x, x_bound),
            (Coordinate2D { x, y}, DimensionBounds2D {x: x_bound, y: y_bound }) =>
                within_bound(x, x_bound) && within_bound(y, y_bound),
            (Coordinate3D { x, y, z },
                DimensionBounds3D { x: x_bound, y: y_bound, z: z_bound }) =>
                within_bound(x, x_bound) && within_bound(y, y_bound) && within_bound(z, z_bound),
            _ => panic!("Dimension mismatch")
        }
    }
    pub fn boundary(self, coord: Coordinate) -> bool {
        match (coord, self) {
            (Coordinate1D { x}, DimensionBounds1D { x: x_bound }) => at_bound(x, x_bound),
            (Coordinate2D { x, y }, DimensionBounds2D {x: x_bound, y: y_bound }) =>
                at_bound(x, x_bound) || at_bound(y, y_bound),
            (Coordinate3D { x, y, z },
                DimensionBounds3D { x: x_bound, y: y_bound, z: z_bound }) =>
                at_bound(x, x_bound) || at_bound(y, y_bound) || at_bound(z, z_bound),
            _ => panic!("Dimension mismatch")
        }
    }
    pub fn x_breadth(&self) -> isize {
        match self {
            DimensionBounds1D { x }
            | DimensionBounds2D { x, .. }
            | DimensionBounds3D { x, .. } => bound_breadth(*x)
        }
    }
    pub fn y_breadth(&self) -> isize {
        match self {
            DimensionBounds1D { .. } => panic!(),
            DimensionBounds2D { y, .. }
            | DimensionBounds3D { y, .. } => bound_breadth(*y)
        }
    }
    pub fn z_breadth(&self) -> isize {
        match self {
            DimensionBounds1D { .. }
            | DimensionBounds2D { .. } => panic!(),
            DimensionBounds3D { z, .. } => bound_breadth(*z)
        }
    }

}
impl IntoIterator for DimensionBounds {
    type Item = Coordinate;
    type IntoIter = DimensionsIter;

    fn into_iter(self) -> Self::IntoIter {
        DimensionsIter::new(self)
    }
}

// FIXME This implementation is kind of awful.
// - Uses different structs for each dimension
// - Nasty, hacky use of loop {}
pub enum DimensionsIter {
    Dimensions1D(DimensionsIter1D),
    Dimensions2D(DimensionsIter2D),
    Dimensions3D(DimensionsIter3D)
}
impl DimensionsIter {
    pub fn new(dimensions: DimensionBounds) -> Self {
        match dimensions {
            DimensionBounds::DimensionBounds1D { x: (low, high) } => {
                Self::Dimensions1D(DimensionsIter1D {
                    queue: (low..=high)
                })
            }
            DimensionBounds::DimensionBounds2D { x: (x_low, x_high), y: (y_low, y_high) } => {
                Self::Dimensions2D(DimensionsIter2D {
                    x_queue: ((x_low+1)..=x_high),
                    y_template: (y_low..=y_high),
                    queue: repeat(x_low).zip(y_low..=y_high)
                })
            }
            DimensionBounds::DimensionBounds3D { x: (x_low, x_high), y: (y_low, y_high), z: (z_low, z_high) } => {
                Self::Dimensions3D(DimensionsIter3D {
                    x_queue: ((x_low+1)..=x_high),
                    current_x: x_low,
                    y_queue: ((y_low+1)..=y_high),
                    y_template: (y_low..=y_high),
                    z_template: (z_low..=z_high),
                    queue: repeat(x_low).zip(repeat(y_low).zip(z_low..=z_high))
                })
            }
        }
    }
}
impl Iterator for DimensionsIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DimensionsIter::Dimensions1D(i) => i.next(),
            DimensionsIter::Dimensions2D(i) => i.next(),
            DimensionsIter::Dimensions3D(i) => i.next()
        }
    }
}

pub struct DimensionsIter1D {
    queue: RangeInclusive<isize>
}
impl Iterator for DimensionsIter1D {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Coordinate1D { x: self.queue.next()? })
    }
}

pub struct DimensionsIter2D {
    x_queue: RangeInclusive<isize>,
    y_template: RangeInclusive<isize>,
    queue: Zip<Repeat<isize>, RangeInclusive<isize>>
}
impl Iterator for DimensionsIter2D {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pt) = self.queue.next() {
                let (x, y) = pt;
                return Some(Coordinate2D { x, y });
            }
            let x = self.x_queue.next()?;
            self.queue = repeat(x).zip(self.y_template.clone());
        }
    }
}
pub struct DimensionsIter3D {
    x_queue: RangeInclusive<isize>,
    current_x: isize,
    y_queue: RangeInclusive<isize>,
    y_template: RangeInclusive<isize>,
    z_template: RangeInclusive<isize>,
    queue: Zip<Repeat<isize>, Zip<Repeat<isize>, RangeInclusive<isize>>>
}
impl Iterator for DimensionsIter3D {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pt) = self.queue.next() {
                let (x, (y, z)) = pt;
                return Some(Coordinate3D {x, y, z});
            }
            if let Some(y) = self.y_queue.next() {
                self.queue = repeat(self.current_x)
                    .zip(repeat(y).zip(self.z_template.clone()));
            } else {
                self.current_x = self.x_queue.next()?;
                self.y_queue = self.y_template.clone();
            }
        }
    }
}

pub struct Runtime {
    current_tick: HashMap<Coordinate, StateId>,
    next_tick: HashMap<Coordinate, StateId>,
    initial_dimensions: DimensionBounds,
    boundary: BoundaryBlock,
    static_state: Option<StateId>,
    default_state: Option<StateId>,
    rules: Rules,
    neighborhood: Neighborhood,
    tick: usize
}
impl Runtime {
    pub fn new(initial_dimensions: DimensionBounds, boundary: BoundaryBlock, states: States, rules: Rules, neighborhood: Neighborhood) -> Self {
        let mut static_state = None;
        if let Some(name) = boundary.is_static() {
            static_state = Some(*states.state_map().get(name).expect("States map is complete."))
        }
        let default_state = states.default_state();
        Self {
            current_tick: HashMap::new(),
            next_tick: HashMap::new(),
            initial_dimensions,
            boundary,
            static_state,
            default_state,
            rules,
            neighborhood,
            tick: 0
        }
    }
    pub fn set_cell(&mut self, coord: Coordinate, state: StateId) -> Option<StateId> {
        self.current_tick.insert(coord, state)
    }
    pub fn set_env(&mut self, environment: HashMap<Coordinate, StateId>) {
        self.current_tick = environment
    }
    pub fn get_env(&self) -> HashMap<Coordinate, StateId> {
        self.current_tick.clone()
    }
    pub fn get_state(&self, coord: Coordinate) -> Option<StateId> {
        self.current_tick.get(&coord).map(|s| *s).or(self.default_state)
    }
    pub fn run_tick(&mut self) {
        let mut schedule = self.current_tick.iter().map(|(c, _)| *c).collect::<Vec<_>>();
        while !schedule.is_empty() {
            let coord = schedule.pop().unwrap();
            let mut neighborhood = Vec::new();
            for neighbor in self.neighborhood.neighbors(coord) {
                if self.boundary.is_finite() && !self.initial_dimensions.contains(neighbor) {
                    continue;
                }
                if self.static_state.is_some() && self.initial_dimensions.boundary(neighbor) {
                    neighborhood.push(self.static_state.unwrap());
                    continue;
                }
                if let Some(s) = self.current_tick.get(&neighbor).map(|s| *s) {
                    neighborhood.push(s);
                } else if !self.boundary.is_finite() {
                    neighborhood.push(self.default_state.unwrap());
                    if self.current_tick.contains_key(&coord) {
                        // Avoid pushing to schedule infinitely by only scheduling neighbors of cells
                        // which existed last tick, and not neighbors of newly created cells
                        schedule.push(neighbor);
                    }
                }
            }
            let state = match self.current_tick.get(&coord) {
                Some(s) => { *s }
                None => { self.default_state.expect("None case should only occur when default state exists.") }
            };
            if let Some(new_state) = self.rules.evaluate(state, neighborhood) {
                if self.default_state.is_none() || new_state != self.default_state.unwrap() {
                    self.next_tick.insert(coord, new_state);
                }
            } else if self.default_state.is_none() || state != self.default_state.unwrap() {
                self.next_tick.insert(coord, state);
            }
        }
        swap(&mut self.current_tick, &mut self.next_tick);
        self.next_tick = HashMap::new();
        self.tick += 1;
    }
    pub fn run(&mut self, ticks: usize) {
        for t in 0..ticks {
            self.run_tick();
        }
    }
    pub fn tick(&self) -> usize {
        self.tick
    }
}

#[cfg(test)]
mod test {
    use crate::lang::parse::blocks::{neighborhood::{Dimension, NeighborhoodRule}, rule::{RuleASTNode, RuleTerminal, RulesBlock, TransitionRule}};

    use super::*;
    use super::DimensionBounds::*;
    use super::Coordinate::*;

    // Dimensions

    #[test]
    fn iterating_dimensions_1d() {
        let dims = DimensionBounds1D { x: (-3, 3) };
        assert_eq!(
            dims.into_iter().collect::<Vec<Coordinate>>(),
            vec!(-3, -2, -1, 0, 1, 2, 3).into_iter().map(|x| Coordinate1D {x}).collect::<Vec<Coordinate>>()
        );
    }

    #[test]
    fn iterating_dimensions_2d() {
        let dims = DimensionBounds2D { x: (-2, 2), y: (-2, 2) };
        assert_eq!(
            dims.into_iter().collect::<Vec<Coordinate>>(),
            vec!(
                (-2, -2), (-2, -1), (-2, 0), (-2, 1), (-2, 2),
                (-1, -2), (-1, -1), (-1, 0), (-1, 1), (-1, 2),
                (0, -2), (0, -1), (0, 0), (0, 1), (0, 2),
                (1, -2), (1, -1), (1, 0), (1, 1), (1, 2),
                (2, -2), (2, -1), (2, 0), (2, 1), (2, 2),
            ).into_iter().map(|(x, y)| Coordinate2D {x, y}).collect::<Vec<Coordinate>>()
        );
    }

    #[test]
    fn iterating_dimensions_3d() {
        let dims = DimensionBounds3D { x: (-2, 2), y: (-2, 2), z: (-2, 2) };
        assert_eq!(
            dims.into_iter().collect::<Vec<Coordinate>>(),
            vec!(
                (-2, -2, -2), (-2, -2, -1), (-2, -2, 0), (-2, -2, 1), (-2, -2, 2),
                (-2, -1, -2), (-2, -1, -1), (-2, -1, 0), (-2, -1, 1), (-2, -1, 2),
                (-2, 0, -2), (-2, 0, -1), (-2, 0, 0), (-2, 0, 1), (-2, 0, 2),
                (-2, 1, -2), (-2, 1, -1), (-2, 1, 0), (-2, 1, 1), (-2, 1, 2),
                (-2, 2, -2), (-2, 2, -1), (-2, 2, 0), (-2, 2, 1), (-2, 2, 2),

                (-1, -2, -2), (-1, -2, -1), (-1, -2, 0), (-1, -2, 1), (-1, -2, 2),
                (-1, -1, -2), (-1, -1, -1), (-1, -1, 0), (-1, -1, 1), (-1, -1, 2),
                (-1, 0, -2), (-1, 0, -1), (-1, 0, 0), (-1, 0, 1), (-1, 0, 2),
                (-1, 1, -2), (-1, 1, -1), (-1, 1, 0), (-1, 1, 1), (-1, 1, 2),
                (-1, 2, -2), (-1, 2, -1), (-1, 2, 0), (-1, 2, 1), (-1, 2, 2),

                (0, -2, -2), (0, -2, -1), (0, -2, 0), (0, -2, 1), (0, -2, 2),
                (0, -1, -2), (0, -1, -1), (0, -1, 0), (0, -1, 1), (0, -1, 2),
                (0, 0, -2), (0, 0, -1), (0, 0, 0), (0, 0, 1), (0, 0, 2),
                (0, 1, -2), (0, 1, -1), (0, 1, 0), (0, 1, 1), (0, 1, 2),
                (0, 2, -2), (0, 2, -1), (0, 2, 0), (0, 2, 1), (0, 2, 2),

                (1, -2, -2), (1, -2, -1), (1, -2, 0), (1, -2, 1), (1, -2, 2),
                (1, -1, -2), (1, -1, -1), (1, -1, 0), (1, -1, 1), (1, -1, 2),
                (1, 0, -2), (1, 0, -1), (1, 0, 0), (1, 0, 1), (1, 0, 2),
                (1, 1, -2), (1, 1, -1), (1, 1, 0), (1, 1, 1), (1, 1, 2),
                (1, 2, -2), (1, 2, -1), (1, 2, 0), (1, 2, 1), (1, 2, 2),

                (2, -2, -2), (2, -2, -1), (2, -2, 0), (2, -2, 1), (2, -2, 2),
                (2, -1, -2), (2, -1, -1), (2, -1, 0), (2, -1, 1), (2, -1, 2),
                (2, 0, -2), (2, 0, -1), (2, 0, 0), (2, 0, 1), (2, 0, 2),
                (2, 1, -2), (2, 1, -1), (2, 1, 0), (2, 1, 1), (2, 1, 2),
                (2, 2, -2), (2, 2, -1), (2, 2, 0), (2, 2, 1), (2, 2, 2),

            ).into_iter().map(|(x, y, z)| Coordinate3D {x, y, z}).collect::<Vec<Coordinate>>()
        );
    }

    #[test]
    fn contains_1d_true() {
        let dims = DimensionBounds::DimensionBounds1D {x: (-1, 1)};
        for x in (-1..=1) {
            assert!(dims.contains(Coordinate1D {x}))
        }
    }

    #[test]
    fn contains_2d_true() {
        let dims = DimensionBounds::DimensionBounds2D {x: (-1, 1), y: (-2, 2)};
        for x in (-1..=1) {
            for y in (-2..=2) {
                assert!(dims.contains(Coordinate2D {x, y}))
            }
        }
    }

    #[test]
    fn contains_3d_true() {
        let dims = DimensionBounds::DimensionBounds3D {x: (-1, 1), y: (-2, 2), z: (-3, 3)};
        for x in (-1..=1) {
            for y in (-2..=2) {
                for z in (-3..=3) {
                    assert!(dims.contains(Coordinate3D { x, y, z }))
                }
            }
        }
    }

    #[test]
    fn contains_1d_false() {
        let dims = DimensionBounds::DimensionBounds1D {x: (-10, 10)};
        assert!(!dims.contains(Coordinate1D {x: -11}));
        assert!(!dims.contains(Coordinate1D {x: 11}));
    }

    #[test]
    fn contains_2d_false() {
        let dims = DimensionBounds::DimensionBounds2D {x: (-1, 1), y: (-2, 2)};
        for x in (-1..1) {
            assert!(!dims.contains(Coordinate2D { x, y: 3 }));
            assert!(!dims.contains(Coordinate2D { x, y: -3 }));
        }
        for y in (-2..2) {
            assert!(!dims.contains(Coordinate2D { x: -2, y }));
            assert!(!dims.contains(Coordinate2D { x: 2, y }));
        }
    }

    #[test]
    fn contains_3d_false() {
        let dims = DimensionBounds::DimensionBounds3D {x: (-1, 1), y: (-2, 2), z: (-3, 3)};
        for x in (-1..1) {
            assert!(!dims.contains(Coordinate3D { x, y: 3, z: 0 }));
            assert!(!dims.contains(Coordinate3D { x, y: -3, z: 0 }));
            assert!(!dims.contains(Coordinate3D { x, y: 0, z: 4 }));
            assert!(!dims.contains(Coordinate3D { x, y: 0, z: -4 }));
        }
        for y in (-2..2) {
            assert!(!dims.contains(Coordinate3D { x: -2, y, z: 0 }));
            assert!(!dims.contains(Coordinate3D { x: 2, y, z: 0 }));
            assert!(!dims.contains(Coordinate3D { x: 0, y, z: 4 }));
            assert!(!dims.contains(Coordinate3D { x: 0, y, z: -4 }));
        }
        for z in (-3..3) {
            assert!(!dims.contains(Coordinate3D { x: -2, y: 0, z }));
            assert!(!dims.contains(Coordinate3D { x: 2, y: 0, z }));
            assert!(!dims.contains(Coordinate3D { x: 0, y: 3, z }));
            assert!(!dims.contains(Coordinate3D { x: 0, y: -3, z }));
        }
    }

    #[test]
    fn boundary_1d_true() {
        let dims = DimensionBounds::DimensionBounds1D {x: (-1, 1)};
        assert!(dims.boundary(Coordinate1D { x: 1 }));
        assert!(dims.boundary(Coordinate1D { x: -1 }));
    }
   
    #[test]
    fn boundary_2d_true() {
        let dims = DimensionBounds::DimensionBounds2D {x: (-1, 1), y: (-2, 2)};
        for x in (-1..=1) {
            assert!(dims.boundary(Coordinate2D { x, y: 2}));
            assert!(dims.boundary(Coordinate2D { x, y: -2}));
        }
        
        for y in (-2..=2) {
            assert!(dims.boundary(Coordinate2D { x: -1, y}));
            assert!(dims.boundary(Coordinate2D { x: 1, y}));
        }
    }

    #[test]
    fn boundary_3d_true() {
        let dims = DimensionBounds::DimensionBounds3D {x: (-1, 1), y: (-2, 2), z: (-3, 3)};
        for x in (-1..=1) {
            for y in (-2..=2) {
                assert!(dims.boundary(Coordinate3D { x, y, z: 3}));
                assert!(dims.boundary(Coordinate3D { x, y, z: -3}));
            }
        }
        for y in (-2..=2) {
            for z in (-3..=3) {
                assert!(dims.boundary(Coordinate3D { x: 1, y, z}));
                assert!(dims.boundary(Coordinate3D { x: -1, y, z}));
            }
        } 
        for x in (-1..=1) {
            for z in (-3..=3) {
                assert!(dims.boundary(Coordinate3D { x, y: 2, z}));
                assert!(dims.boundary(Coordinate3D { x, y: -2, z}));
            }
        }
    }
    
    #[test]
    fn boundary_1d_false() {
        let dims = DimensionBounds1D { x: (-3, 3) };
        for x in (-2..=2) {
            assert!(!dims.boundary(Coordinate1D { x }));
        }
    }
    
    #[test]
    fn boundary_2d_false() {
        let dims = DimensionBounds::DimensionBounds2D { x: (-2, 2), y: (-2, 2) };
        for x in (-1..=1) {
            for y in (-1..=1) {
                assert!(!dims.boundary(Coordinate2D { x, y }))
            }
        }
    }
    
    #[test]
    fn boundary_3d_false() {
        let dims = DimensionBounds::DimensionBounds3D { x: (-2, 2), y: (-2, 2), z: (-2, 2) };
        for x in (-1..=1) {
            for y in (-1..=1) {
                for z in (-1..=1) {
                    assert!(!dims.boundary(Coordinate3D { x, y, z }))
                }
            }
        }
    }

    // Runtime

    #[test]
    fn runtime_oscillate_single_cell_1d() {
        let mut state_map = HashMap::new();
        state_map.insert("A".into(), 0);
        state_map.insert("B".into(), 1);

        let mut rt = Runtime::new(
            DimensionBounds1D { x: (-0, 0)},
            BoundaryBlock::Void,
            States {
                num_states: 2,
                state_map: state_map.clone(),
                default: None,
            },
            Rules::new(
                RulesBlock::new(
                    vec!(
                        TransitionRule {
                            from: "A".into(),
                            to: "B".into(),
                            root: Box::new(RuleASTNode::GreaterThan {
                                lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(2))),
                                rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1))),
                            })
                        }
                    )
                ),
                &state_map
            ),
            Neighborhood::new(vec!())
        );
        rt.set_cell(Coordinate1D { x: 0 }, 0);
        assert_eq!(rt.get_state(Coordinate1D { x: 0 }), Some(0));
        rt.run_tick();
        assert_eq!(rt.get_state(Coordinate1D { x: 0 }), Some(1));
    }
    
    #[test]
    fn runtime_propogate_infinite_boundary_1d() {
        let mut state_map = HashMap::new();
        state_map.insert("A".into(), 0);
        state_map.insert("B".into(), 1);

        let mut rt = Runtime::new(
            DimensionBounds1D { x: (-5, 5) },
            BoundaryBlock::Infinite,
            States {
                num_states: 2,
                state_map: state_map.clone(),
                default: Some(0),
            },
            Rules::new(
                RulesBlock::new(
                    vec!(
                        TransitionRule {
                            from: "A".into(),
                            to: "B".into(),
                            root: Box::new(RuleASTNode::GreaterThanOrEqualTo {
                                lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1))),
                            })
                        }
                    )
                ),
                &state_map
            ),
            Neighborhood::new(vec!(
                NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1 },
            ))
        );
        rt.set_cell(Coordinate1D { x: 0}, 1);

        for t in (1..10) {
            assert_eq!(rt.get_state(Coordinate1D { x: t }), Some(0));
            assert_eq!(rt.get_state(Coordinate1D { x: -t }), Some(0));
            rt.run_tick();
            for x in (0..t) {
                assert_eq!(rt.get_state(Coordinate1D { x }), Some(1));
                assert_eq!(rt.get_state(Coordinate1D { x: -x }), Some(1));
            }
        }
    }
    
    #[test]
    fn runtime_does_not_alter_static_boundary_cells_1d() {
        let mut state_map = HashMap::new();
        state_map.insert("A".into(), 0);
        state_map.insert("B".into(), 1);

        let mut rt = Runtime::new(
            DimensionBounds1D { x: (-1, 1) },
            BoundaryBlock::Static(Some("A".into())),
            States {
                num_states: 2,
                state_map: state_map.clone(),
                default: Some(0),
            },
            Rules::new(
                RulesBlock::new(
                    vec!(
                        TransitionRule {
                            from: "A".into(),
                            to: "B".into(),
                            root: Box::new(RuleASTNode::GreaterThanOrEqualTo {
                                lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("B".into()))),
                                rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1))),
                            })
                        }
                    )
                ),
                &state_map
            ),
            Neighborhood::new(vec!(
                NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1 },
            ))
        );
        rt.set_cell(Coordinate1D { x: 0}, 1);

        assert_eq!(rt.get_state(Coordinate1D { x: 1 }), Some(0));
        assert_eq!(rt.get_state(Coordinate1D { x: -1 }), Some(0));
        rt.run_tick();
        assert_eq!(rt.get_state(Coordinate1D { x: 1 }), Some(0));
        assert_eq!(rt.get_state(Coordinate1D { x: -1 }), Some(0));
    }

}
