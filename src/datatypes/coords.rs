use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, RangeInclusive};

use itertools::{Itertools, Product};

#[cfg(test)]
pub mod coords_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimensionality {
    OneDimensional,
    TwoDimensional,
    ThreeDimensional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dimension {
    X,
    Y,
    Z,
    All,
}
impl IntoIterator for Dimension {
    type Item = Dimension;
    type IntoIter = DimensionIterator;

    fn into_iter(self) -> Self::IntoIter {
        DimensionIterator::new(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DimensionIterator {
    dimension: Dimension,
    current_dim: usize,
    done: bool,
}
impl DimensionIterator {
    pub fn new(dimension: Dimension) -> Self {
        Self {
            dimension,
            current_dim: 0,
            done: false,
        }
    }
}
impl Iterator for DimensionIterator {
    type Item = Dimension;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.dimension {
            Dimension::All => {
                let d = match self.current_dim {
                    0 => Some(Dimension::X),
                    1 => Some(Dimension::Y),
                    2 => Some(Dimension::Z),
                    _ => {
                        self.done = true;
                        None
                    }
                };
                self.current_dim += 1;
                d
            }
            _ => {
                self.done = true;
                Some(self.dimension)
            }
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Coordinate1D {
    x: isize,
}

impl Coordinate1D {
    pub fn new(x: isize) -> Self {
        Self { x }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Coordinate2D {
    x: isize,
    y: isize,
}

impl Coordinate2D {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Coordinate3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate3D {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

pub trait Coordinate:
    Default + Hash + PartialEq + Eq + Copy + Clone + Debug + Add<Self, Output = Self>
{
    fn x(&self) -> isize;
    fn y(&self) -> isize;
    fn z(&self) -> isize;
    fn set_x(&mut self, x: isize);
    fn set_y(&mut self, y: isize);
    fn set_z(&mut self, z: isize);
    fn set_all(&mut self, value: isize);
    fn set(&mut self, dimension: Dimension, value: isize) {
        match dimension {
            Dimension::X => self.set_x(value),
            Dimension::Y => self.set_y(value),
            Dimension::Z => self.set_z(value),
            Dimension::All => self.set_all(value),
        }
    }
    fn offset(self, dimension: Dimension, value: isize) -> OffsetIterator<Self> {
        OffsetIterator::new(self, dimension, value)
    }
    fn dimensionality() -> Dimensionality;
}

impl Coordinate for Coordinate1D {
    fn x(&self) -> isize {
        self.x
    }
    fn y(&self) -> isize {
        panic!("Cannot get Y value from 1D coordinate.");
    }
    fn z(&self) -> isize {
        panic!("Cannot get z value from 1D coordinate.");
    }
    fn set_x(&mut self, x: isize) {
        self.x = x;
    }
    fn set_y(&mut self, _y: isize) {
        panic!("Cannot set Y value on 1D coordinate.")
    }
    fn set_z(&mut self, _z: isize) {
        panic!("Cannot set Z value on 1D coordinate.")
    }
    fn set_all(&mut self, value: isize) {
        self.x = value;
    }
    fn dimensionality() -> Dimensionality {
        Dimensionality::OneDimensional
    }
}
impl Add<Coordinate1D> for Coordinate1D {
    type Output = Self;

    fn add(self, rhs: Coordinate1D) -> Self::Output {
        Self { x: self.x + rhs.x }
    }
}
impl Coordinate for Coordinate2D {
    fn x(&self) -> isize {
        self.x
    }
    fn y(&self) -> isize {
        self.y
    }
    fn z(&self) -> isize {
        panic!("Cannot get Z value from 2D coordinate.");
    }
    fn set_x(&mut self, x: isize) {
        self.x = x;
    }
    fn set_y(&mut self, y: isize) {
        self.y = y;
    }
    fn set_z(&mut self, _z: isize) {
        panic!("Cannot set Z value on 2D coordinate.")
    }
    fn set_all(&mut self, value: isize) {
        self.x = value;
        self.y = value;
    }
    fn dimensionality() -> Dimensionality {
        Dimensionality::TwoDimensional
    }
}
impl Add<Coordinate2D> for Coordinate2D {
    type Output = Self;

    fn add(self, rhs: Coordinate2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Coordinate for Coordinate3D {
    fn x(&self) -> isize {
        self.x
    }
    fn y(&self) -> isize {
        self.y
    }
    fn z(&self) -> isize {
        self.z
    }
    fn set_x(&mut self, x: isize) {
        self.x = x;
    }
    fn set_y(&mut self, y: isize) {
        self.y = y;
    }
    fn set_z(&mut self, z: isize) {
        self.z = z;
    }
    fn set_all(&mut self, value: isize) {
        self.x = value;
        self.y = value;
        self.z = value;
    }
    fn dimensionality() -> Dimensionality {
        Dimensionality::ThreeDimensional
    }
}
impl Add<Coordinate3D> for Coordinate3D {
    type Output = Self;

    fn add(self, rhs: Coordinate3D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Clone)]
pub struct OffsetIterator<C: Coordinate> {
    coord: C,
    dim_iter: DimensionIterator,
    value: isize,
    last_dim: Dimension,
}

impl<C: Coordinate> OffsetIterator<C> {
    pub fn new(coord: C, dimension: Dimension, value: isize) -> Self {
        Self {
            coord,
            dim_iter: dimension.into_iter(),
            value,
            // Avoid setting dimensions that C doesn't have when setting all
            // dimensions
            // TODO hacky, could be implemented in a cleaner way. Not a high priority
            // though.
            last_dim: match C::dimensionality() {
                Dimensionality::OneDimensional => Dimension::X,
                Dimensionality::TwoDimensional => Dimension::Y,
                Dimensionality::ThreeDimensional => Dimension::Z,
            },
        }
    }
}

impl<C: Coordinate> Iterator for OffsetIterator<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        let dim = self.dim_iter.next()?;
        if dim > self.last_dim {
            return None;
        }
        let mut c = self.coord.clone();
        c.set(dim, self.value);
        Some(c)
    }
}

pub trait ClosedSet<C: Coordinate, I: Iterator<Item = C>>:
    Debug + Copy + IntoIterator<Item = C, IntoIter = I>
{
    fn contains(&self, coord: C) -> Contains;
    fn within(&self, coord: C) -> bool {
        self.contains(coord) == Contains::Within
    }
    fn outside(&self, coord: C) -> bool {
        self.contains(coord) == Contains::Outside
    }
    fn on_edge(&self, coord: C) -> bool {
        self.contains(coord) == Contains::OnEdge
    }
}

// TODO hack to let FixedGrid accept a ClosedSet
pub trait CoordinateBounds<C: Coordinate>: IntoIterator<Item = C> {}

impl CoordinateBounds<Coordinate1D> for BoundingBox1D {}
impl CoordinateBounds<Coordinate2D> for BoundingBox2D {}
impl CoordinateBounds<Coordinate3D> for BoundingBox3D {}
impl CoordinateBounds<Coordinate1D> for Vec<Coordinate1D> {}
impl CoordinateBounds<Coordinate2D> for Vec<Coordinate2D> {}
impl CoordinateBounds<Coordinate3D> for Vec<Coordinate3D> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contains {
    Within,
    Outside,
    OnEdge,
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox1D {
    low: isize,
    high: isize,
}
impl BoundingBox1D {
    pub fn new(low: isize, high: isize) -> Self {
        Self { low, high }
    }
}
impl ClosedSet<Coordinate1D, BoundingBox1DIterator> for BoundingBox1D {
    fn contains(&self, coord: Coordinate1D) -> Contains {
        match coord {
            coord if self.low < coord.x() && self.high > coord.x() => Contains::Within,
            coord if self.low == coord.x() || self.high == coord.x() => Contains::OnEdge,
            _ => Contains::Outside,
        }
    }
}
impl IntoIterator for BoundingBox1D {
    type Item = Coordinate1D;
    type IntoIter = BoundingBox1DIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoundingBox1DIterator::new(self.low, self.high)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox2D {
    x: (isize, isize),
    y: (isize, isize),
}

impl BoundingBox2D {
    pub fn new(x: (isize, isize), y: (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl ClosedSet<Coordinate2D, BoundingBox2DIterator> for BoundingBox2D {
    fn contains(&self, coord: Coordinate2D) -> Contains {
        if self.x.0 <= coord.x()
            && self.x.1 >= coord.x()
            && self.y.0 <= coord.y()
            && self.y.1 >= coord.y()
        {
            if self.x.0 == coord.x()
                || self.x.1 == coord.x()
                || self.y.0 == coord.y()
                || self.y.1 == coord.y()
            {
                Contains::OnEdge
            } else {
                Contains::Within
            }
        } else {
            Contains::Outside
        }
    }
}
impl IntoIterator for BoundingBox2D {
    type Item = Coordinate2D;
    type IntoIter = BoundingBox2DIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoundingBox2DIterator::new(self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox3D {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl BoundingBox3D {
    pub fn new(x: (isize, isize), y: (isize, isize), z: (isize, isize)) -> Self {
        Self { x, y, z }
    }
}

impl ClosedSet<Coordinate3D, BoundingBox3DIterator> for BoundingBox3D {
    fn contains(&self, coord: Coordinate3D) -> Contains {
        if self.x.0 <= coord.x()
            && self.x.1 >= coord.x()
            && self.y.0 <= coord.y()
            && self.y.1 >= coord.y()
            && self.z.0 <= coord.z()
            && self.z.1 >= coord.z()
        {
            if self.x.0 == coord.x()
                || self.x.1 == coord.x()
                || self.y.0 == coord.y()
                || self.y.1 == coord.y()
                || self.z.0 == coord.z()
                || self.z.1 == coord.z()
            {
                Contains::OnEdge
            } else {
                Contains::Within
            }
        } else {
            Contains::Outside
        }
    }
}
impl IntoIterator for BoundingBox3D {
    type Item = Coordinate3D;
    type IntoIter = BoundingBox3DIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoundingBox3DIterator::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox1DIterator {
    range: RangeInclusive<isize>,
}
impl BoundingBox1DIterator {
    pub fn new(low: isize, high: isize) -> Self {
        Self { range: low..=high }
    }
}
impl Iterator for BoundingBox1DIterator {
    type Item = Coordinate1D;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.range.next()?;
        Some(Coordinate1D::new(i))
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox2DIterator {
    product: Product<RangeInclusive<isize>, RangeInclusive<isize>>,
}
impl BoundingBox2DIterator {
    pub fn new(x: (isize, isize), y: (isize, isize)) -> Self {
        let x = x.0..=x.1;
        let y = y.0..=y.1;
        Self {
            product: x.cartesian_product(y),
        }
    }
}
impl Iterator for BoundingBox2DIterator {
    type Item = Coordinate2D;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.product.next()?;
        Some(Coordinate2D::new(x, y))
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox3DIterator {
    product: Product<Product<RangeInclusive<isize>, RangeInclusive<isize>>, RangeInclusive<isize>>,
}
impl BoundingBox3DIterator {
    pub fn new(x: (isize, isize), y: (isize, isize), z: (isize, isize)) -> Self {
        let x = x.0..=x.1;
        let y = y.0..=y.1;
        let z = z.0..=z.1;
        Self {
            product: x.cartesian_product(y).cartesian_product(z),
        }
    }
}
impl Iterator for BoundingBox3DIterator {
    type Item = Coordinate3D;

    fn next(&mut self) -> Option<Self::Item> {
        let ((x, y), z) = self.product.next()?;
        Some(Coordinate3D::new(x, y, z))
    }
}

// TODO finish
#[derive(Debug, Clone, Copy)]
pub struct Circle2D {
    center: Coordinate2D,
    radius: isize,
}

impl Circle2D {
    pub fn new(center: Coordinate2D, radius: isize) -> Self {
        Self { center, radius }
    }
}

impl ClosedSet<Coordinate2D, Circle2DIterator> for Circle2D {
    fn contains(&self, coord: Coordinate2D) -> Contains {
        let (coord_x, coord_y) = (coord.x(), coord.y());
        let (center_x, center_y) = (self.center.x(), self.center.y());
        match (coord_x * center_x + coord_y * center_y).cmp(&self.radius) {
            Ordering::Equal => Contains::OnEdge,
            Ordering::Less => Contains::Within,
            Ordering::Greater => Contains::Outside,
        }
    }
}
impl IntoIterator for Circle2D {
    type Item = Coordinate2D;
    type IntoIter = Circle2DIterator;

    fn into_iter(self) -> Self::IntoIter {
        Circle2DIterator::new(self.center, self.radius)
    }
}

#[derive(Debug, Clone)]
pub struct Circle2DIterator {}
impl Circle2DIterator {
    pub fn new(center: Coordinate2D, radius: isize) -> Self {
        Self {}
    }
}
impl Iterator for Circle2DIterator {
    type Item = Coordinate2D;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
