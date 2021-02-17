use std::fmt::Debug;
use std::hash::Hash;

use crate::datatypes::coords::Coordinate;

pub trait Identifer: Hash + Eq + Copy + Clone + Debug {}
impl<T: Coordinate> Identifer for T {}
