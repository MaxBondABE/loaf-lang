use std::fmt::Debug;

use crate::datatypes::state::State;

pub trait Neighborhood<S>: Debug + Clone {
    fn count(&self, state: S) -> usize;
}

impl<S: State> Neighborhood<S> for Vec<S> {
    fn count(&self, state: S) -> usize {
        self.iter().filter(|s| **s == state).count()
    }
}
