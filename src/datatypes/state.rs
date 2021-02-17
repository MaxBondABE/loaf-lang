use std::fmt::Debug;
use std::hash::Hash;

/// Trait for types used to store cell state information
pub trait State: Copy + Clone + Ord + Eq + Hash + Default + Debug {}
impl State for usize {}
impl State for u8 {}
