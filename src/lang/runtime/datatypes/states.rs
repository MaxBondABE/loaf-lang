use std::collections::HashMap;

use crate::lang::{parse::blocks::state::{Attribute, StatesBlock}, runtime::{StateId, StateMap}};

pub struct States {
    num_states: StateId,
    state_map: StateMap,
    default: Option<StateId>
}
impl States {
    pub fn new(num_states: StateId, state_map: StateMap, default: Option<StateId>) -> Self { Self { num_states, state_map, default } }

    pub fn from_block(states_block: StatesBlock) -> Self {
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

