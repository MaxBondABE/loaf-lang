// Unit Tests

#[cfg(test)]
pub mod dimension_iterator_tests {
    use super::*;

    #[test]
    fn iterating_over_single_dimension_yields_that_dimension_only() {
        let dims = [Dimension::X, Dimension::Y, Dimension::Z];
        for dim in &dims {
            let mut iter = dim.into_iter();
            assert_eq!(iter.next(), Some(*dim));
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn iterating_over_all_dimenions_yields_each_individual_dimension() {
        assert_eq!(
            Dimension::All.into_iter().collect::<Vec<_>>(),
            vec!(Dimension::X, Dimension::Y, Dimension::Z)
        )
    }
}

#[cfg(test)]
pub mod coordinates_tests {
    use super::*;

    /// 1D

    #[test]
    fn coord_1d_default_is_origin() {
        assert_eq!(Coordinate1D::default(), Coordinate1D::new(0));
    }

    #[test]
    fn coord_1d_set_x_works() {
        let mut coord = Coordinate1D::default();
        coord.set_x(100);
        assert_eq!(coord, Coordinate1D::new(100));
    }

    #[test]
    #[should_panic]
    fn coord_1d_set_y_panics() {
        let mut coord = Coordinate1D::default();
        coord.set_y(0);
    }

    #[test]
    #[should_panic]
    fn coord_1d_set_z_panics() {
        let mut coord = Coordinate1D::default();
        coord.set_z(0);
    }

    #[test]
    fn coord_1d_set_all_works() {
        let mut coord = Coordinate1D::default();
        coord.set_all(100);
        assert_eq!(coord, Coordinate1D::new(100));
    }

    #[test]
    fn coord_1d_get_x_works() {
        let mut coord = Coordinate1D::default();
        assert_eq!(coord.x(), 0);
        coord.set_x(100);
        assert_eq!(coord.x(), 100);
    }

        
    #[test]
    #[should_panic]
    fn coord_1d_get_y_panics() {
        Coordinate1D::default().y();
    }
    
    #[test]
    #[should_panic]
    fn coord_1d_get_z_panics() {
        Coordinate1D::default().z();
    }

    #[test]
    fn coord_1d_has_correct_dimensionality() {
        assert_eq!(Coordinate1D::dimensionality(), Dimensionality::OneDimensional);
    }


    #[test]
    fn coord_1d_offset_x_works() {
        assert_eq!(
            Coordinate1D::default().offset(Dimension::X, 100).collect::<Vec<_>>(),
            vec!(Coordinate1D::new(100))
        )
    }

    #[test]
    fn coord_1d_offset_all_works() {
        assert_eq!(
            Coordinate1D::default().offset(Dimension::All, 100).collect::<Vec<_>>(),
            vec!(Coordinate1D::new(100))
        )
    }

    #[test]
    fn coord_1d_addition_works() {
        assert_eq!(
            Coordinate1D::new(50) + Coordinate1D::new(25),
            Coordinate1D::new(75)
        )
    }

    /// 2D

    #[test]
    fn coord_2d_default_is_origin() {
        assert_eq!(Coordinate2D::default(), Coordinate2D::new(0, 0));
    }

    #[test]
    fn coord_2d_set_x_works() {
        let mut coord = Coordinate2D::default();
        coord.set_x(100);
        assert_eq!(coord, Coordinate2D::new(100, 0));
    }

    #[test]
    fn coord_2d_set_y_works() {
        let mut coord = Coordinate2D::default();
        coord.set_y(100);
        assert_eq!(coord, Coordinate2D::new(0, 100));
    }

    #[test]
    #[should_panic]
    fn coord_2d_set_z_panics() {
        let mut coord = Coordinate2D::default();
        coord.set_z(0);
    }

    #[test]
    fn coord_2d_set_all_works() {
        let mut coord = Coordinate2D::default();
        coord.set_all(100);
        assert_eq!(coord, Coordinate2D::new(100, 100));
    }

    #[test]
    fn coord_2d_get_x_works() {
        let mut coord = Coordinate2D::default();
        assert_eq!(coord.x(), 0);
        coord.set_x(100);
        assert_eq!(coord.x(), 100);
    }

    #[test]
    fn coord_2d_get_y_works() {
        let mut coord = Coordinate2D::default();
        assert_eq!(coord.y(), 0);
        coord.set_y(100);
        assert_eq!(coord.y(), 100);
    }

    #[test]
    #[should_panic]
    fn coord_2d_get_z_panics() {
        Coordinate2D::default().z();
    }

    #[test]
    fn coord_2d_has_correct_dimensionality() {
        assert_eq!(Coordinate2D::dimensionality(), Dimensionality::TwoDimensional);
    }

    #[test]
    fn coord_2d_offset_x_works() {
        assert_eq!(
            Coordinate2D::default().offset(Dimension::X, 100).collect::<Vec<_>>(),
            vec!(Coordinate2D::new(100, 0))
        )
    }

    #[test]
    fn coord_2d_offset_y_works() {
        assert_eq!(
            Coordinate2D::default().offset(Dimension::Y, 100).collect::<Vec<_>>(),
            vec!(Coordinate2D::new(0, 100))
        )
    }

    #[test]
    fn coord_2d_offset_all_works() {
        assert_eq!(
            Coordinate2D::default().offset(Dimension::All, 100).collect::<Vec<_>>(),
            vec!(Coordinate2D::new(100, 0), Coordinate2D::new(0, 100))
        )
    }

    #[test]
    fn coord_2d_addition_works() {
        assert_eq!(
            Coordinate2D::new(0, 50) + Coordinate2D::new(10, 25),
            Coordinate2D::new(10, 75)
        )
    }

    /// 3D

    #[test]
    fn coord_3d_default_is_origin() {
        assert_eq!(Coordinate3D::default(), Coordinate3D::new(0, 0, 0));
    }

    #[test]
    fn coord_3d_set_x_works() {
        let mut coord = Coordinate3D::default();
        coord.set_x(100);
        assert_eq!(coord, Coordinate3D::new(100, 0, 0));
    }

    #[test]
    fn coord_3d_set_y_works() {
        let mut coord = Coordinate3D::default();
        coord.set_y(100);
        assert_eq!(coord, Coordinate3D::new(0, 100, 0));
    }

    #[test]
    fn coord_3d_set_z_works() {
        let mut coord = Coordinate3D::default();
        coord.set_z(100);
        assert_eq!(coord, Coordinate3D::new(0, 0, 100));
    }

    #[test]
    fn coord_3d_set_all_works() {
        let mut coord = Coordinate3D::default();
        coord.set_all(100);
        assert_eq!(coord, Coordinate3D::new(100, 100, 100));
    }

    #[test]
    fn coord_3d_get_x_works() {
        let mut coord = Coordinate3D::default();
        assert_eq!(coord.x(), 0);
        coord.set_x(100);
        assert_eq!(coord.x(), 100);
    }

    #[test]
    fn coord_3d_get_y_works() {
        let mut coord = Coordinate3D::default();
        assert_eq!(coord.y(), 0);
        coord.set_y(100);
        assert_eq!(coord.y(), 100);
    }

    #[test]
    fn coord_3d_get_z_works() {
        let mut coord = Coordinate3D::default();
        assert_eq!(coord.z(), 0);
        coord.set_z(100);
        assert_eq!(coord.z(), 100);
    }

    #[test]
    fn coord_3d_has_correct_dimensionality() {
        assert_eq!(Coordinate3D::dimensionality(), Dimensionality::ThreeDimensional);
    }


    #[test]
    fn coord_3d_offset_x_works() {
        assert_eq!(
            Coordinate3D::default().offset(Dimension::X, 100).collect::<Vec<_>>(),
            vec!(Coordinate3D::new(100, 0, 0))
        )
    }

    #[test]
    fn coord_3d_offset_y_works() {
        assert_eq!(
            Coordinate3D::default().offset(Dimension::Y, 100).collect::<Vec<_>>(),
            vec!(Coordinate3D::new(0, 100, 0))
        )
    }

    #[test]
    fn coord_3d_offset_all_works() {
        assert_eq!(
            Coordinate3D::default().offset(Dimension::All, 100).collect::<Vec<_>>(),
            vec!(Coordinate3D::new(100, 0, 0), Coordinate3D::new(0, 100, 0), Coordinate3D::new(0, 0, 100))
        )
    }

    #[test]
    fn coord_3d_addition_works() {
        assert_eq!(
            Coordinate3D::new(0, 50, 100) + Coordinate3D::new(10, 25, -10),
            Coordinate3D::new(10, 75, 90)
        )
    }
        
    // The set() is function only tested for 3d since a.) the implementation
    // is the same for all coords, b.) it uses functionality tested earlier and
    // c.) it is unlikely to break or change

    #[test]
    fn coord_set_by_dimension_works_for_x() {
        let mut coord = Coordinate3D::default();
        coord.set(Dimension::X, 100);
        assert_eq!(coord, Coordinate3D::new(100, 0, 0));
    }

    #[test]
    fn coord_set_by_dimension_works_for_y() {
        let mut coord = Coordinate3D::default();
        coord.set(Dimension::Y, 100);
        assert_eq!(coord, Coordinate3D::new(0, 100, 0));
    }

    #[test]
    fn coord_set_by_dimension_works_for_z() {
        let mut coord = Coordinate3D::default();
        coord.set(Dimension::Z, 100);
        assert_eq!(coord, Coordinate3D::new(0, 0, 100));
    }

    #[test]
    fn coord_set_by_dimension_works_for_all() {
        let mut coord = Coordinate3D::default();
        coord.set(Dimension::All, 100);
        assert_eq!(coord, Coordinate3D::new(100, 100, 100));
    }
}

#[cfg(test)]
pub mod closed_set_tests {
    use super::*;

    /// 1D

    #[test]
    fn bounding_box_1d_within() {
        let bb = BoundingBox1D::new(1, 5);
        for x in 2..=4 {
            assert!(bb.within(Coordinate1D::new(x)));
        }
    }

    #[test]
    fn bounding_box_1d_outside() {
        let bb = BoundingBox1D::new(1, 5);
        assert!(bb.outside(Coordinate1D::new(0)));
        assert!(bb.outside(Coordinate1D::new(6)));
    }

    #[test]
    fn bounding_box_1d_on_edge() {
        let bb = BoundingBox1D::new(1, 5);
        assert!(bb.on_edge(Coordinate1D::new(1)));
        assert!(bb.on_edge(Coordinate1D::new(5)));
    }

    /// 2D

    #[test]
    fn bounding_box_2d_within() {
        let bb = BoundingBox2D::new((1, 5), (-2, 2));
        for (x, y) in (2..=4).cartesian_product(-1..1) {
            assert!(bb.within(Coordinate2D::new(x,y)));
        }
    }

    #[test]
    fn bounding_box_2d_outside() {
        let bb = BoundingBox2D::new((1, 5), (-1, 1));
        for x in 1..=5 {
            assert!(bb.outside(Coordinate2D::new(x, -2)));
            assert!(bb.outside(Coordinate2D::new(x, 2)));
        }
        for y in -1..=1 {
            assert!(bb.outside(Coordinate2D::new(0, y)));
            assert!(bb.outside(Coordinate2D::new(6, y)));
        }
    }

    #[test]
    fn bounding_box_2d_on_edge() {
        let bb = BoundingBox2D::new((1, 5), (-1, 1));
        for x in 1..=5 {
            assert!(bb.on_edge(Coordinate2D::new(x, -1)));
            assert!(bb.on_edge(Coordinate2D::new(x, 1)));
        }
        for y in -1..=1 {
            assert!(bb.on_edge(Coordinate2D::new(1, y)));
            assert!(bb.on_edge(Coordinate2D::new(5, y)));
        }
    }

    /// 3D

    #[test]
    fn bounding_box_3d_within() {
        let bb = BoundingBox3D::new((1, 5), (-2, 2), (5, 10));
        for ((x, y), z) in (2..=4).cartesian_product(-1..1).cartesian_product(6..=9) {
            assert!(bb.within(Coordinate3D::new(x,y,z)));
        }
    }

    #[test]
    fn bounding_box_3d_outside() {
        let bb = BoundingBox3D::new((1, 5), (-2, 2), (5, 10));
        for (x, y) in (1..=5).cartesian_product(-2..2) {
            assert!(bb.outside(Coordinate3D::new(x, y, 4)));
            assert!(bb.outside(Coordinate3D::new(x, y, 11)));
        }
        for (y, z) in (-2..=2).cartesian_product(5..=10) {
            assert!(bb.outside(Coordinate3D::new(0, y, z)));
            assert!(bb.outside(Coordinate3D::new(6, y, z)));
        }
        for (x, z) in (1..=5).cartesian_product(5..=10) {
            assert!(bb.outside(Coordinate3D::new(x, -3, z)));
            assert!(bb.outside(Coordinate3D::new(x, 3, z)));
        }
    }

    #[test]
    fn bounding_box_3d_on_edge() {
        let bb = BoundingBox3D::new((1, 5), (-2, 2), (5, 10));
        for (x, y) in (2..=4).cartesian_product(-1..1) {
            assert!(bb.on_edge(Coordinate3D::new(x, y, 5)));
            assert!(bb.on_edge(Coordinate3D::new(x, y, 10)));
        }
        for (y, z) in (-1..=1).cartesian_product(6..=9) {
            assert!(bb.on_edge(Coordinate3D::new(1, y, z)));
            assert!(bb.on_edge(Coordinate3D::new(5, y, z)));
        }
        for (x, z) in (2..=4).cartesian_product(6..=9) {
            assert!(bb.on_edge(Coordinate3D::new(x, -2, z)));
            assert!(bb.on_edge(Coordinate3D::new(x, 2, z)));
        }
    }
}

