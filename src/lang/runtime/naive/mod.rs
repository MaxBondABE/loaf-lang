use crate::lang::runtime::naive::Coordinate::{Coordinate1D, Coordinate2D, Coordinate3D};

mod ops;

pub(crate) type StateId = usize;
type StateMap = HashMap<String, StateId>;

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
