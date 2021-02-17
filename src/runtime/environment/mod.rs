pub mod naive;

use std::collections::HashMap;

use crate::datatypes::ident::Identifer;
use crate::datatypes::neighborhood::Neighborhood;
use crate::datatypes::state::State;

pub trait Environment<I: Identifer, S: State, N: Neighborhood<S>, Schedule: IntoIterator<Item = I>>
{
    fn set_state(&mut self, ident: I, state: S);
    fn get_state(&self, ident: I) -> Option<S>;
    fn get_neighborhood(&self, ident: I) -> Option<N>;

    fn schedule(&mut self, ident: I);
    fn deschedule(&mut self, ident: I);
    fn get_schedule(&self) -> Schedule; // TODO iterator

    fn snapshot(&self) -> HashMap<I, S>;
    fn tick(&mut self);
}
