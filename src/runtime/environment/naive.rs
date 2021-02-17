use std::collections::HashMap;
use std::convert::identity;
use std::marker::PhantomData;
use std::mem::swap;

use crate::datatypes::coords::{Coordinate, CoordinateBounds};
use crate::runtime::environment::Environment;

pub struct FixedGrid<C: Coordinate, CB: CoordinateBounds<C>> {
    current_tick: HashMap<C, usize>,
    next_tick: HashMap<C, usize>,
    neighborhood: Box<[C]>,
    phantom: PhantomData<CB>,
}
impl<C: Coordinate, CB: CoordinateBounds<C>> FixedGrid<C, CB> {
    pub fn new(neighborhood: Box<[C]>, bounds: CB) -> Self {
        let current_tick: HashMap<C, usize> = bounds.into_iter().map(|c| (c, 0)).collect();
        let next_tick = HashMap::with_capacity(current_tick.capacity());
        Self {
            current_tick,
            next_tick,
            neighborhood,
            phantom: PhantomData,
        }
    }

    pub fn from_hashmap(neighborhood: Box<[C]>, hashmap: HashMap<C, usize>, bounds: CB) -> Self {
        let mut current_tick = hashmap;
        for coord in bounds {
            if !current_tick.contains_key(&coord) {
                current_tick.insert(coord, 0);
            }
        }
        let next_tick = HashMap::with_capacity(current_tick.capacity());
        Self {
            current_tick,
            next_tick,
            neighborhood,
            phantom: PhantomData,
        }
    }
}
impl<C: Coordinate, CB: CoordinateBounds<C>> Environment<C, usize, Vec<usize>, Vec<C>>
    for FixedGrid<C, CB>
{
    fn set_state(&mut self, coord: C, state: usize) {
        self.next_tick.insert(coord, state);
    }

    fn get_state(&self, coord: C) -> Option<usize> {
        self.current_tick.get(&coord).map(|s| *s)
    }

    fn get_neighborhood(&self, coord: C) -> Option<Vec<usize>> {
        if !self.current_tick.contains_key(&coord) {
            return None;
        }
        Some(
            self.neighborhood
                .iter()
                .map(|c| coord + *c)
                .map(|c| self.get_state(c))
                .filter_map(identity)
                .collect(),
        )
    }

    fn schedule(&mut self, _ident: C) {
        panic!("NaiveGrid is a fixed-size environment -- cannot schedule or deschedule");
    }
    fn deschedule(&mut self, _ident: C) {
        panic!("NaiveGrid is a fixed-size environment -- cannot schedule or deschedule");
    }

    fn get_schedule(&self) -> Vec<C> {
        self.current_tick.keys().map(|c| *c).collect()
    }

    fn snapshot(&self) -> HashMap<C, usize> {
        self.current_tick.clone()
    }

    fn tick(&mut self) {
        let Self {
            current_tick,
            next_tick,
            ..
        } = self;
        for (k, v) in current_tick.drain() {
            if !next_tick.contains_key(&k) {
                next_tick.insert(k, v);
            }
        }
        swap(&mut self.current_tick, &mut self.next_tick);
    }
}

#[cfg(test)]
pub mod fixed_grid_test {
    use super::*;
    use crate::datatypes::coords::Coordinate1D;

    #[test]
    fn set_state_inserts_into_next_tick() {
        let coord = Coordinate1D::new(0);
        let mut env = FixedGrid::<Coordinate1D, Vec<Coordinate1D>>::new(
            vec![].into_boxed_slice(),
            vec![coord],
        );
        env.set_state(coord, 1);
        assert_eq!(env.current_tick.get(&coord), Some(&0));
        assert_eq!(env.next_tick.get(&coord), Some(&1));
    }

    #[test]
    fn get_state_retrieves_from_current_tick() {
        let coord = Coordinate1D::new(0);
        let mut env = FixedGrid::<Coordinate1D, Vec<Coordinate1D>>::new(
            vec![].into_boxed_slice(),
            vec![coord],
        );
        env.current_tick.insert(coord, 0);
        env.next_tick.insert(coord, 1);
        assert_eq!(env.get_state(coord), Some(0));
    }

    #[test]
    fn tick_propogates_changes() {
        let coord1 = Coordinate1D::new(0);
        let coord2 = Coordinate1D::new(1);
        let mut env = FixedGrid::<Coordinate1D, Vec<Coordinate1D>>::new(
            vec![].into_boxed_slice(),
            vec![coord1, coord2],
        );
        env.current_tick.insert(coord1, 0);
        env.current_tick.insert(coord2, 0);
        env.set_state(coord2, 2);
        env.tick();
        assert_eq!(env.get_state(coord2), Some(2));
    }

    #[test]
    fn tick_preserves_unchanged_coords() {
        let coord1 = Coordinate1D::new(0);
        let coord2 = Coordinate1D::new(1);
        let mut env = FixedGrid::<Coordinate1D, Vec<Coordinate1D>>::new(
            vec![].into_boxed_slice(),
            vec![coord1, coord2],
        );
        env.current_tick.insert(coord1, 0);
        env.current_tick.insert(coord2, 0);
        env.set_state(coord2, 2);
        env.tick();
        assert_eq!(env.get_state(coord1), Some(0));
    }

    #[test]
    fn setting_state_does_not_change_neighborhood() {
        let coord1 = Coordinate1D::new(0);
        let coord2 = Coordinate1D::new(1);
        let neighborhood = vec![coord2];
        let mut env = FixedGrid::<Coordinate1D, Vec<Coordinate1D>>::new(
            neighborhood.into_boxed_slice(),
            vec![coord1, coord2],
        );
        let a = env.get_neighborhood(coord1);
        env.set_state(coord2, 2);
        let b = env.get_neighborhood(coord1);
        assert_eq!(a, b);
    }

    #[test]
    fn get_neighborhood_retrieves_adjacent_cells() {
        let coord1 = Coordinate1D::new(0);
        let coord2 = Coordinate1D::new(1);
        let coord3 = Coordinate1D::new(-1);
        let neighborhood = vec![coord2, coord3];
        let mut env = FixedGrid::<Coordinate1D, Vec<Coordinate1D>>::new(
            neighborhood.into_boxed_slice(),
            vec![coord1, coord2],
        );
        env.current_tick.insert(coord2, 1);
        env.current_tick.insert(coord3, 2);
        assert_eq!(env.get_neighborhood(coord1), Some(vec!(1, 2)))
    }

    #[test]
    fn get_neighborhood_works_on_edges() {
        let coord1 = Coordinate1D::new(0);
        let coord2 = Coordinate1D::new(1);
        let coord3 = Coordinate1D::new(-1);
        let neighborhood = vec![coord2, coord3];
        let mut env = FixedGrid::<Coordinate1D, Vec<Coordinate1D>>::new(
            neighborhood.into_boxed_slice(),
            vec![coord1, coord2],
        );
        env.current_tick.insert(coord2, 1);
        env.current_tick.insert(coord3, 2);
        assert_eq!(env.get_neighborhood(coord3), Some(vec!(0)))
    }
}
