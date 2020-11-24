use std::collections::HashMap;

pub mod neighborhood;
pub mod rules;

type StateId = usize;
type FromState = StateId;
type ToState = StateId;
type StateMap = HashMap<String, StateId>;