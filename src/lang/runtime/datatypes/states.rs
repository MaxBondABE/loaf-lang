use std::collections::HashMap;
use image::Rgb;

use crate::lang::{parse::blocks::state::{Attribute, StatesBlock}, runtime::{StateId, StateMap}};

// TODO put this in some constants.rs
const DEFAULT_COLOR: (u8, u8, u8) = (0xff, 0xff, 0xff);

pub struct States {
    num_states: StateId,
    name_map: StateMap,
    color_map: HashMap<StateId, Rgb<u8>>,
    default: Option<StateId>
}
impl States {
    pub fn new(num_states: StateId, name_map: StateMap, color_map: HashMap<StateId, Rgb<u8>>, default: Option<StateId>) -> Self {
        Self { num_states, name_map, color_map, default }
    }

    pub fn from_block(states_block: StatesBlock) -> Self {
        let states_block = states_block.into_map();
        let num_states = states_block.iter().count();
        let mut default = None;
        let mut name_map = HashMap::new();
        let mut color_map = HashMap::new();
        for (state_id, (name, attributes)) in states_block.into_iter().enumerate() {
            name_map.insert(name, state_id);
            if attributes.iter().find(|a| **a == Attribute::Default).is_some() {
                default = Some(state_id);
            }
            let (r, g, b) = attributes.iter().find(|a| a.is_color())
                .map(|a| match a {
                    Attribute::Color(opt) => opt.expect("is_color() should prevent None values"),
                    _ => unreachable!()
                }).unwrap_or(DEFAULT_COLOR);
            color_map.insert(
                state_id,
                image::Rgb([r, g, b])
            );
        }
        Self {
            num_states,
            name_map,
            color_map,
            default
        }
    }
    pub fn name_map(&self) -> StateMap {
        self.name_map.clone()
    }
    pub fn color_map(&self) -> HashMap<StateId, Rgb<u8>> {
        self.color_map.clone()
    }
    pub fn default_state(&self) -> Option<usize> { self.default }
}

