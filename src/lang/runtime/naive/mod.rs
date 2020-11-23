use crate::lang::runtime::naive::Coordinate::{Coordinate1D, Coordinate2D, Coordinate3D};

mod ops;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Coordinate {
    Coordinate1D {x: isize},
    Coordinate2D {x: isize, y: isize},
    Coordinate3D {x: isize, y: isize, z: isize}
}
impl Coordinate {
    pub fn add_x(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { x } => {
                Coordinate1D {x: x + magnitude}
            }
            Coordinate2D { x, y } => {
                Coordinate2D { x: x + magnitude, y}
            }
            Coordinate3D { x, y, z } => {
                Coordinate3D { x: x + magnitude, y, z}
            }
        }
    }
    pub fn add_y(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { .. } => panic!("Illegal coordinate operation: 1D coordinate has no Y value"),
            Coordinate2D { x, y } => {
                Coordinate2D { x, y: y + magnitude }
            }
            Coordinate3D { x, y, z } => {
                Coordinate3D { x, y: y + magnitude, z }
            }
        }
    }
    pub fn add_z(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { .. } => panic!("Illegal coordinate operation: 1D coordinate has no Z value"),
            Coordinate2D { .. }  => panic!("Illegal coordinate operation: 2D coordinate has no Z value"),
            Coordinate3D { x, y, z } => {
                Coordinate3D { x, y, z: z + magnitude }
            }
        }
    }
    pub fn add_all(self, magnitude: isize) -> Vec<Coordinate> {
        match self {
            Coordinate::Coordinate1D { x } => {
                vec!(Coordinate1D {x: x + magnitude})
            }
            Coordinate::Coordinate2D { x, y } => {
                vec!(
                    Coordinate2D {x: x + magnitude, y},
                    Coordinate2D {x, y: y + magnitude}
                )
            }
            Coordinate::Coordinate3D { x, y, z } => {
                vec!(
                    Coordinate3D {x: x + magnitude, y, z},
                    Coordinate3D {x, y: y + magnitude, z},
                    Coordinate3D {x, y, z: z + magnitude},
                )
            }
        }
    }
    pub fn sub_x(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { x } => {
                Coordinate1D {x: x - magnitude}
            }
            Coordinate2D { x, y } => {
                Coordinate2D { x: x - magnitude, y }
            }
            Coordinate3D { x, y, z } => {
                Coordinate3D { x: x - magnitude, y, z }
            }
        }
    }
    pub fn sub_y(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { .. } => panic!("Illegal coordinate operation: 1D coordinate has no Y value"),
            Coordinate2D { x, y } => {
                Coordinate2D { x, y: y - magnitude }
            }
            Coordinate3D { x, y, z } => {
                Coordinate3D { x, y: y - magnitude, z }
            }
        }
    }
    pub fn sub_z(self, magnitude: isize) -> Self {
        match self {
            Coordinate1D { .. } => panic!("Illegal coordinate operation: 1D coordinate has no Z value"),
            Coordinate2D { .. } => panic!("Illegal coordinate operation: 2D coordinate has no Z value"),
            Coordinate3D { x, y, z } => {
                Coordinate3D { x, y, z: z - magnitude }
            }
        }
    }
    pub fn sub_all(self, magnitude: isize) -> Vec<Coordinate> {
        match self {
            Coordinate::Coordinate1D { x } => {
                vec!(Coordinate1D {x: x - magnitude})
            }
            Coordinate::Coordinate2D { x, y } => {
                vec!(
                    Coordinate2D {x: x - magnitude, y},
                    Coordinate2D {x, y: y - magnitude}
                )
            }
            Coordinate::Coordinate3D { x, y, z } => {
                vec!(
                    Coordinate3D {x: x - magnitude, y, z},
                    Coordinate3D {x, y: y - magnitude, z},
                    Coordinate3D {x, y, z: z - magnitude},
                )
            }
        }
    }
}
/*
type StateId = u8;
type NumStates = u8;

struct Axis<T> {
    negative: Vec<T>,
    positive: Vec<T>
}
impl Axis<T> {
    fn new(&self) -> Self {
        Self { negative: Vec::new(), positive: Vec::new()}
    }
    fn get(&self, idx: isize) -> Option<T> {
        if idx >= 0 {
            self.positive.get(idx)
        } else {
            self.negative.get(-1 * idx)
        }
    }
}

enum Environment<'tick> {
    Grid1D(Axis<Cell<'tick>>),
    Grid2D(Axis<Axis<Cell<'tick>>>),
    Grid3D(Axis<Axis<Cell<'tick>>>)
}

enum Cell<'tick> {
    Realized(RealizedCell<'tick>),
    Placeholder(PlaceholderCell)
}

struct RealizedCell<'tick> {
    state: StateId,
    next_state: Option<StateId>,
    scheduled: bool
}

struct PlaceholderCell {
    state: StateId,
    scheduled: bool
}*/