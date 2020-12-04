use std::collections::HashMap;
use crate::lang::runtime::StateId;

pub mod neighborhood;
pub mod rules;

type FromState = StateId;
type ToState = StateId;
