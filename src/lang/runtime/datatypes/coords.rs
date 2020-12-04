use std::{iter::Repeat, iter::Zip, iter::repeat, ops::RangeInclusive};

use self::Coordinate::*;
use DimensionBounds::*;

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

pub type Bound = (isize, isize);
fn within_bound(coord: isize, bound: Bound) -> bool {
    let (low, high) = bound;
    low <= coord && coord <= high
}
fn at_bound(coord: isize, bound: Bound) -> bool {
    let (low, high) = bound;
    low == coord || coord == high
}
fn bound_breadth(bound: Bound) -> isize {
    let (low, high) = bound;
    high - low + 1
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DimensionBounds {
    DimensionBounds1D { x: Bound },
    DimensionBounds2D { x: Bound, y: Bound },
    DimensionBounds3D { x: Bound, y: Bound, z: Bound }
}
impl DimensionBounds {
    pub fn contains(self, coord: Coordinate) -> bool {
        match (coord, self) {
            (Coordinate1D { x }, DimensionBounds1D { x: x_bound }) => within_bound(x, x_bound),
            (Coordinate2D { x, y}, DimensionBounds2D {x: x_bound, y: y_bound }) =>
                within_bound(x, x_bound) && within_bound(y, y_bound),
            (Coordinate3D { x, y, z },
                DimensionBounds3D { x: x_bound, y: y_bound, z: z_bound }) =>
                within_bound(x, x_bound) && within_bound(y, y_bound) && within_bound(z, z_bound),
            _ => panic!("Dimension mismatch")
        }
    }
    pub fn boundary(self, coord: Coordinate) -> bool {
        match (coord, self) {
            (Coordinate1D { x}, DimensionBounds1D { x: x_bound }) => at_bound(x, x_bound),
            (Coordinate2D { x, y }, DimensionBounds2D {x: x_bound, y: y_bound }) =>
                at_bound(x, x_bound) || at_bound(y, y_bound),
            (Coordinate3D { x, y, z },
                DimensionBounds3D { x: x_bound, y: y_bound, z: z_bound }) =>
                at_bound(x, x_bound) || at_bound(y, y_bound) || at_bound(z, z_bound),
            _ => panic!("Dimension mismatch")
        }
    }
    pub fn x_breadth(&self) -> isize {
        match self {
            DimensionBounds1D { x }
            | DimensionBounds2D { x, .. }
            | DimensionBounds3D { x, .. } => bound_breadth(*x)
        }
    }
    pub fn y_breadth(&self) -> isize {
        match self {
            DimensionBounds1D { .. } => panic!(),
            DimensionBounds2D { y, .. }
            | DimensionBounds3D { y, .. } => bound_breadth(*y)
        }
    }
    pub fn z_breadth(&self) -> isize {
        match self {
            DimensionBounds1D { .. }
            | DimensionBounds2D { .. } => panic!(),
            DimensionBounds3D { z, .. } => bound_breadth(*z)
        }
    }

}
impl IntoIterator for DimensionBounds {
    type Item = Coordinate;
    type IntoIter = DimensionsIter;

    fn into_iter(self) -> Self::IntoIter {
        DimensionsIter::new(self)
    }
}

// FIXME This implementation is kind of awful.
// - Uses different structs for each dimension
// - Nasty, hacky use of loop {}
pub enum DimensionsIter {
    Dimensions1D(DimensionsIter1D),
    Dimensions2D(DimensionsIter2D),
    Dimensions3D(DimensionsIter3D)
}
impl DimensionsIter {
    pub fn new(dimensions: DimensionBounds) -> Self {
        match dimensions {
            DimensionBounds::DimensionBounds1D { x: (low, high) } => {
                Self::Dimensions1D(DimensionsIter1D {
                    queue: (low..=high)
                })
            }
            DimensionBounds::DimensionBounds2D { x: (x_low, x_high), y: (y_low, y_high) } => {
                Self::Dimensions2D(DimensionsIter2D {
                    x_queue: ((x_low+1)..=x_high),
                    y_template: (y_low..=y_high),
                    queue: repeat(x_low).zip(y_low..=y_high)
                })
            }
            DimensionBounds::DimensionBounds3D { x: (x_low, x_high), y: (y_low, y_high), z: (z_low, z_high) } => {
                Self::Dimensions3D(DimensionsIter3D {
                    x_queue: ((x_low+1)..=x_high),
                    current_x: x_low,
                    y_queue: ((y_low+1)..=y_high),
                    y_template: (y_low..=y_high),
                    z_template: (z_low..=z_high),
                    queue: repeat(x_low).zip(repeat(y_low).zip(z_low..=z_high))
                })
            }
        }
    }
}
impl Iterator for DimensionsIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DimensionsIter::Dimensions1D(i) => i.next(),
            DimensionsIter::Dimensions2D(i) => i.next(),
            DimensionsIter::Dimensions3D(i) => i.next()
        }
    }
}

pub struct DimensionsIter1D {
    queue: RangeInclusive<isize>
}
impl Iterator for DimensionsIter1D {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Coordinate1D { x: self.queue.next()? })
    }
}

pub struct DimensionsIter2D {
    x_queue: RangeInclusive<isize>,
    y_template: RangeInclusive<isize>,
    queue: Zip<Repeat<isize>, RangeInclusive<isize>>
}
impl Iterator for DimensionsIter2D {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pt) = self.queue.next() {
                let (x, y) = pt;
                return Some(Coordinate2D { x, y });
            }
            let x = self.x_queue.next()?;
            self.queue = repeat(x).zip(self.y_template.clone());
        }
    }
}
pub struct DimensionsIter3D {
    x_queue: RangeInclusive<isize>,
    current_x: isize,
    y_queue: RangeInclusive<isize>,
    y_template: RangeInclusive<isize>,
    z_template: RangeInclusive<isize>,
    queue: Zip<Repeat<isize>, Zip<Repeat<isize>, RangeInclusive<isize>>>
}
impl Iterator for DimensionsIter3D {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pt) = self.queue.next() {
                let (x, (y, z)) = pt;
                return Some(Coordinate3D {x, y, z});
            }
            if let Some(y) = self.y_queue.next() {
                self.queue = repeat(self.current_x)
                    .zip(repeat(y).zip(self.z_template.clone()));
            } else {
                self.current_x = self.x_queue.next()?;
                self.y_queue = self.y_template.clone();
            }
        }
    }
}
