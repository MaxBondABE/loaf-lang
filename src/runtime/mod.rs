pub mod environment;
pub mod neighborhood;
pub mod state;

use std::collections::HashMap;
use std::marker::PhantomData;

use crate::datatypes::ident::Identifer;
use crate::datatypes::neighborhood::Neighborhood;
use crate::datatypes::state::State;
use crate::runtime::environment::Environment;
use crate::runtime::state::Ruleset;

pub trait Runtime<Delta, E> {
    fn run_tick(&mut self) -> Delta;
    fn run_ticks(&mut self, ticks: usize);
    fn environment(&self) -> &E;
}

pub struct SynchronousRuntime<S: State, N: Neighborhood<S>, E, Schedule> {
    ruleset: Ruleset<S, N>,
    environment: E,
    _marker: PhantomData<(Schedule,)>,
}
impl<S: State, N: Neighborhood<S>, E, Schedule> SynchronousRuntime<S, N, E, Schedule> {
    pub fn new(ruleset: Ruleset<S, N>, environment: E) -> Self {
        Self {
            ruleset,
            environment,
            _marker: PhantomData,
        }
    }
}
impl<
        I: Identifer,
        S: State,
        N: Neighborhood<S>,
        Schedule: IntoIterator<Item = I>,
        E: Environment<I, S, N, Schedule>,
    > Runtime<HashMap<I, S>, E> for SynchronousRuntime<S, N, E, Schedule>
{
    // TODO allow for different types of deltas
    fn run_tick(&mut self) -> HashMap<I, S> {
        let mut delta = HashMap::new();
        for cell in self.environment.get_schedule() {
            if let Some(state) = self.ruleset.transition(
                self.environment
                    .get_state(cell)
                    .expect("All scheduled cells should have a state"),
                self.environment
                    .get_neighborhood(cell)
                    .expect("All scheduled celss should have a neighborhood"),
            ) {
                self.environment.set_state(cell, state);
                delta.insert(cell, state);
            }
        }
        self.environment.tick();
        delta
    }

    fn run_ticks(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.run_tick();
        }
    }

    fn environment(&self) -> &E {
        &self.environment
    }
}

// TODO parrallel runtime using rayon
// pub struct SynchronousRuntime<I: Identifer, S: State, N: Neighborhood<S>, Delta: IntoIterator<Item=(I, S)>, Schedule: IntoIterator<Item=I>, E: Environment<I, S, N, Delta, Schedule>> {
