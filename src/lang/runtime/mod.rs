use std::collections::HashMap;

pub mod naive;
pub mod ops;
pub mod datatypes;

// TODO make types generic over StateId
// TODO move to datetypes::states
pub(crate) type StateId = usize;
pub(crate) type StateMap = HashMap<String, StateId>;

