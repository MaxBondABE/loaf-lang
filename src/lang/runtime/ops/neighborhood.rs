use crate::lang::parse::blocks::neighborhood::{Dimension, NeighborhoodBlock, NeighborhoodRule};
use crate::lang::runtime::datatypes::coords::Coordinate;
use std::slice::Iter;

pub struct Neighborhood {
    rules: Vec<NeighborhoodRule>
}
impl Neighborhood {
    pub fn new(rules: Vec<NeighborhoodRule>) -> Self {
        println!("rules {:?}", &rules);
        Self { rules }
    }
    pub fn from_block(block: NeighborhoodBlock) -> Self {
        Self::new(block.into_rules())
    }
    pub fn neighbors(&self, starting_coordinate: Coordinate) -> NeighborhoodIter {
        NeighborhoodIter::new(starting_coordinate, self.rules.iter())
    }
}

pub struct NeighborhoodIter<'a> {
    starting_coordinate: Coordinate,
    rules_iter: Iter<'a, NeighborhoodRule>,
    queue: Vec<Coordinate>,
    done: bool
}
impl<'a> NeighborhoodIter<'a> {
    pub fn new(starting_coordinate: Coordinate, rules_iter: Iter<'a, NeighborhoodRule>) -> Self {
        Self {
            starting_coordinate,
            rules_iter,
            queue: Vec::with_capacity(2),
            done: false
        }
    }
}
impl Iterator for NeighborhoodIter<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None
        }
        if !self.queue.is_empty() {
            return self.queue.pop()
        }
        let rule = self.rules_iter.next();
        if rule.is_none() {
            self.done = true;
            return None;
        }

        match rule.unwrap() {
            NeighborhoodRule::DirectedEdge {dimension, magnitude} => {
                // The positive/negative direction is encoded in the sign of magnitude, so we only
                // need to use addition
                match dimension {
                    Dimension::X => {
                        Some(self.starting_coordinate.add_x(*magnitude))
                    },
                    Dimension::Y => {
                        Some(self.starting_coordinate.add_y(*magnitude))
                    }
                    Dimension::Z => {
                        Some(self.starting_coordinate.add_z(*magnitude))
                    }
                    Dimension::All => {
                        self.queue = self.starting_coordinate.add_all(*magnitude);
                        self.queue.pop()
                    }
                }
            }
            NeighborhoodRule::UndirectedEdge { dimension, magnitude } => {
                match dimension {
                    Dimension::X => {
                        self.queue.push(self.starting_coordinate.add_x(*magnitude));
                        Some(self.starting_coordinate.sub_x(*magnitude))
                    }
                    Dimension::Y => {
                        self.queue.push(self.starting_coordinate.add_y(*magnitude));
                        Some(self.starting_coordinate.sub_y(*magnitude))
                    }
                    Dimension::Z => {
                        self.queue.push(self.starting_coordinate.add_z(*magnitude));
                        Some(self.starting_coordinate.sub_z(*magnitude))
                    }
                    Dimension::All => {
                        self.queue = self.starting_coordinate.add_all(*magnitude);
                        self.queue.append(&mut self.starting_coordinate.sub_all(*magnitude));
                        self.queue.pop()
                    }
                }
            }
            NeighborhoodRule::UndirectedCircle { .. } => unimplemented!(),
            NeighborhoodRule::Compound (child_rules) => {
                // FIXME filled with hacks, hacks on hacks.
                assert!(!child_rules.is_empty()); // FIXME can't handle empty rules - filter them out in prior step
                let mut rule_queue: Vec<NeighborhoodRule> = child_rules.clone().into_iter().rev().collect();
                let rule = rule_queue.pop().expect("Should contain at least 1 rule.");

                let mut coord_queue: Vec<Coordinate> = NeighborhoodIter::new(self.starting_coordinate, vec!(rule).iter()).collect();

                for rule in rule_queue {
                    let mut new_coords = Vec::new();
                    for coord in coord_queue {
                        for resulting_coord in NeighborhoodIter::new(coord, vec!(rule.clone()).iter()) {
                            new_coords.push(resulting_coord);
                        }
                    }
                    coord_queue = new_coords;
                }
                self.queue.append(&mut coord_queue);
                self.queue.pop()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::runtime::datatypes::coords::Coordinate::*;
    use crate::lang::parse::blocks::neighborhood::MOORE_RULES;
    use std::collections::HashSet;

    const COORD_1D: Coordinate = Coordinate1D { x: 10 };
    const COORD_2D: Coordinate = Coordinate2D { x: 10, y: 20 };
    const COORD_3D: Coordinate = Coordinate3D { x: 10, y: 20, z: 30 };

    // TODO rulesets with multiple rules

    // 1D

    #[test]
    pub fn directed_edge_positive_1d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::X, magnitude: 1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_1D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate1D {x: 11})
        )
    }

    #[test]
    pub fn directed_edge_negative_1d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::X, magnitude: -1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_1D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate1D {x: 9})
        )
    }

    #[test]
    pub fn undirected_edge_1d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_1D).collect()),
            to_set(vec!(
                Coordinate1D {x: 9},
                Coordinate1D {x: 11}
            ))
        );
    }

    #[test]
    pub fn directed_edge_positive_1d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::All, magnitude: 1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_1D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate1D {x: 11})
        )
    }

    #[test]
    pub fn directed_edge_negative_1d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::All, magnitude: -1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_1D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate1D {x: 9})
        )
    }

    #[test]
    pub fn undirected_edge_1d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::All, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_1D).collect()),
            to_set(vec!(
                Coordinate1D {x: 9},
                Coordinate1D {x: 11}
            ))
        )
    }

    // 2D

    #[test]
    pub fn directed_edge_positive_2d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::X, magnitude: 1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_2D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate2D {x: 11, y: 20})
        )
    }

    #[test]
    pub fn directed_edge_negative_2d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::X, magnitude: -1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_2D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate2D {x: 9, y: 20})
        )
    }

    #[test]
    pub fn undirected_edge_2d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_2D).collect()),
            to_set(vec!(
                Coordinate2D { x: 9, y: 20 },
                Coordinate2D { x: 11, y: 20 })
            )
        );
    }

    #[test]
    pub fn directed_edge_positive_2d_y() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::Y, magnitude: 1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_2D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate2D {x: 10, y: 21})
        )
    }

    #[test]
    pub fn directed_edge_negative_2d_y() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::Y, magnitude: -1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_2D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate2D {x: 10, y: 19})
        )
    }

    #[test]
    pub fn undirected_edge_2d_y() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::Y, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_2D).collect()),
            to_set(vec!(
                Coordinate2D { x: 10, y: 19 },
                Coordinate2D { x: 10, y: 21 },
            ))
        );
    }

    #[test]
    pub fn directed_edge_positive_2d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::All, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_2D).collect()),
            to_set(vec!(
                Coordinate2D { x: 11, y: 20 },
                Coordinate2D { x: 10, y: 21 }
            ))
        )
    }

    #[test]
    pub fn directed_edge_negative_2d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::All, magnitude: -1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_2D).collect()),
            to_set(vec!(
                Coordinate2D { x: 9, y: 20 },
                Coordinate2D { x: 10, y: 19 }
            ))
        )
    }

    #[test]
    pub fn undirected_edge_2d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::All, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_2D).collect()),
            to_set(vec!(
                Coordinate2D { x: 9, y: 20 },
                Coordinate2D { x: 11, y: 20 },
                Coordinate2D { x: 10, y: 19 },
                Coordinate2D { x: 10, y: 21 },
            ))
        );
    }

    // 3D

    #[test]
    pub fn directed_edge_positive_3d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::X, magnitude: 1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_3D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate3D {x: 11, y: 20, z: 30})
        )
    }

    #[test]
    pub fn directed_edge_negative_3d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::X, magnitude: -1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_3D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate3D {x: 9, y: 20, z: 30})
        )
    }

    #[test]
    pub fn undirected_edge_3d_x() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_3D).collect()),
            to_set(vec!(
                Coordinate3D { x: 9, y: 20, z: 30 },
                Coordinate3D { x: 11, y: 20, z: 30 })
            )
        );
    }

    #[test]
    pub fn directed_edge_positive_3d_y() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::Y, magnitude: 1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_3D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate3D {x: 10, y: 21, z: 30})
        )
    }

    #[test]
    pub fn directed_edge_negative_3d_y() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::Y, magnitude: -1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_3D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate3D {x: 10, y: 19, z: 30})
        )
    }

    #[test]
    pub fn undirected_edge_3d_y() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::Y, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_3D).collect()),
            to_set(vec!(
                Coordinate3D { x: 10, y: 19, z: 30 },
                Coordinate3D { x: 10, y: 21, z: 30 },
            ))
        );
    }

    #[test]
    pub fn directed_edge_positive_3d_z() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::Z, magnitude: 1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_3D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate3D {x: 10, y: 20, z: 31})
        )
    }

    #[test]
    pub fn directed_edge_negative_3d_z() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::Z, magnitude: -1}
        ));
        assert_eq!(
            neighborhood.neighbors(COORD_3D).collect::<Vec<Coordinate>>(),
            vec!(Coordinate3D {x: 10, y: 20, z: 29})
        )
    }

    #[test]
    pub fn undirected_edge_3d_z() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::Z, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_3D).collect()),
            to_set(vec!(
                Coordinate3D { x: 10, y: 20, z: 29 },
                Coordinate3D { x: 10, y: 20, z: 31 },
            ))
        );
    }

    #[test]
    pub fn directed_edge_positive_3d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::All, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_3D).collect()),
            to_set(vec!(
                Coordinate3D { x: 11, y: 20, z: 30 },
                Coordinate3D { x: 10, y: 21, z: 30 },
                Coordinate3D { x: 10, y: 20, z: 31 }
            ))
        )
    }

    #[test]
    pub fn directed_edge_negative_3d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::DirectedEdge { dimension: Dimension::All, magnitude: -1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_3D).collect()),
            to_set(vec!(
                Coordinate3D { x: 9, y: 20, z: 30 },
                Coordinate3D { x: 10, y: 19, z: 30 },
                Coordinate3D { x: 10, y: 20, z: 29 }
            ))
        )
    }

    #[test]
    pub fn undirected_edge_3d_all() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::UndirectedEdge { dimension: Dimension::All, magnitude: 1}
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(COORD_3D).collect()),
            to_set(vec!(
                Coordinate3D { x: 9, y: 20, z: 30 },
                Coordinate3D { x: 11, y: 20, z: 30 },
                Coordinate3D { x: 10, y: 21, z: 30 },
                Coordinate3D { x: 10, y: 19, z: 30 },
                Coordinate3D { x: 10, y: 20, z: 29 },
                Coordinate3D { x: 10, y: 20, z: 31 }
            ))
        );
    }

    #[test]
    pub fn compound() {
        let neighborhood = Neighborhood::new(vec!(
            NeighborhoodRule::Compound(vec!(
                    NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1},
                    NeighborhoodRule::UndirectedEdge { dimension: Dimension::Y, magnitude: 1},
            )),
            NeighborhoodRule::Compound(vec!(
                    NeighborhoodRule::UndirectedEdge { dimension: Dimension::Y, magnitude: 1},
                    NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1},
            ))
        ));
        assert_eq!(
            to_set(neighborhood.neighbors(Coordinate2D { x: 0, y: 0}).collect()),
            to_set(vec!(
                Coordinate2D { x: 1, y: 1 },
                Coordinate2D { x: -1, y: 1 },
                Coordinate2D { x: -1, y: -1 },
                Coordinate2D { x: 1, y: -1 }
            ))
        );
    }

    pub fn moore_2d() {
        let neighborhood = Neighborhood::new(MOORE_RULES.clone());
        assert_eq!(
            to_set(neighborhood.neighbors(Coordinate2D { x: 0, y: 0}).collect()),
            to_set(vec!(
                // Von Neumann neighbors
                Coordinate2D { x: 1, y: 0},
                Coordinate2D { x: 0, y: 1},
                Coordinate2D { x: -1, y: 0},
                Coordinate2D { x: 0, y: -1},

                // Corners
                Coordinate2D { x: 1, y: 1 },
                Coordinate2D { x: -1, y: 1 },
                Coordinate2D { x: -1, y: -1 },
                Coordinate2D { x: 1, y: -1 }
            ))
        );
        assert_eq!(
            neighborhood.neighbors(Coordinate2D { x: 0, y: 0}).collect::<Vec<_>>().len(),
            vec!(
                // Von Neumann neighbors
                Coordinate2D { x: 1, y: 0},
                Coordinate2D { x: 0, y: 1},
                Coordinate2D { x: -1, y: 0},
                Coordinate2D { x: 0, y: -1},

                // Corners
                Coordinate2D { x: 1, y: 1 },
                Coordinate2D { x: -1, y: 1 },
                Coordinate2D { x: -1, y: -1 },
                Coordinate2D { x: 1, y: -1 }
            ).len()
        );
        assert_eq!(
            neighborhood.neighbors(Coordinate2D { x: 0, y: 0}).collect::<Vec<_>>().len(),
            8
        )
    }

    // Util

    /// Convert a `Vec` to a `HashSet` so they can be compared w/o regards to order (which is
    /// unspecified) while still getting good error output.
    fn to_set(v: Vec<Coordinate>) -> HashSet<Coordinate> {
        let mut h = HashSet::new();
        for i in v {
            h.insert(i);
        }
        h
    }

}
