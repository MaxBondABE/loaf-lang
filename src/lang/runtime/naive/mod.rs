use std::collections::HashMap;
use crate::lang::parse::blocks::boundary::BoundaryBlock;
use crate::lang::runtime::StateId;
use crate::lang::runtime::StateMap;
use crate::lang::runtime::ops::rules::Rules;
use crate::lang::runtime::ops::neighborhood::Neighborhood;
use crate::lang::parse::blocks::state::{StatesBlock, Attribute};
use crate::lang::runtime::datatypes::coords::Coordinate;
use crate::lang::runtime::datatypes::coords::DimensionBounds;
use crate::lang::runtime::datatypes::coords::Coordinate::*;
use crate::lang::runtime::datatypes::coords::DimensionBounds::*;
use crate::lang::runtime::datatypes::states::States;
use std::slice::Iter;
use std::mem::swap;

pub struct Runtime {
    current_tick: HashMap<Coordinate, StateId>,
    next_tick: HashMap<Coordinate, StateId>,
    initial_dimensions: DimensionBounds,
    boundary: BoundaryBlock,
    static_state: Option<StateId>,
    default_state: Option<StateId>,
    rules: Rules,
    neighborhood: Neighborhood,
    tick: usize
}
impl Runtime {
    pub fn new(initial_dimensions: DimensionBounds, boundary: BoundaryBlock, states: States, rules: Rules, neighborhood: Neighborhood) -> Self {
        let mut static_state = None;
        if let Some(name) = boundary.is_static() {
            static_state = Some(*states.name_map().get(name).expect("States map is complete."))
        }
        let default_state = states.default_state();
        Self {
            current_tick: HashMap::new(),
            next_tick: HashMap::new(),
            initial_dimensions,
            boundary,
            static_state,
            default_state,
            rules,
            neighborhood,
            tick: 0
        }
    }
    pub fn set_cell(&mut self, coord: Coordinate, state: StateId) -> Option<StateId> {
        self.current_tick.insert(coord, state)
    }
    pub fn set_env(&mut self, environment: HashMap<Coordinate, StateId>) {
        self.current_tick = environment
    }
    pub fn get_env(&self) -> HashMap<Coordinate, StateId> {
        self.current_tick.clone()
    }
    pub fn get_state(&self, coord: Coordinate) -> Option<StateId> {
        self.current_tick.get(&coord).map(|s| *s).or(self.default_state)
    }
    pub fn run_tick(&mut self) {
        let mut schedule = self.current_tick.iter().map(|(c, _)| *c).collect::<Vec<_>>();
        while !schedule.is_empty() {
            let coord = schedule.pop().unwrap();
            let mut neighborhood = Vec::new();
            for neighbor in self.neighborhood.neighbors(coord) {
                if self.boundary.is_finite() && !self.initial_dimensions.contains(neighbor) {
                    continue;
                }
                if self.static_state.is_some() && self.initial_dimensions.boundary(neighbor) {
                    neighborhood.push(self.static_state.unwrap());
                    continue;
                }
                if let Some(s) = self.current_tick.get(&neighbor).map(|s| *s) {
                    neighborhood.push(s);
                } else if !self.boundary.is_finite() {
                    neighborhood.push(self.default_state.unwrap());
                    if self.current_tick.contains_key(&coord) {
                        // Avoid pushing to schedule infinitely by only scheduling neighbors of cells
                        // which existed last tick, and not neighbors of newly created cells
                        schedule.push(neighbor);
                    }
                }
            }
            let state = match self.current_tick.get(&coord) {
                Some(s) => { *s }
                None => { self.default_state.expect("None case should only occur when default state exists.") }
            };
            if let Some(new_state) = self.rules.evaluate(state, neighborhood) {
                if self.default_state.is_none() || new_state != self.default_state.unwrap() {
                    self.next_tick.insert(coord, new_state);
                }
            } else if self.default_state.is_none() || state != self.default_state.unwrap() {
                self.next_tick.insert(coord, state);
            }
        }
        swap(&mut self.current_tick, &mut self.next_tick);
        self.next_tick = HashMap::new();
        self.tick += 1;
    }
    pub fn run(&mut self, ticks: usize) {
        for t in 0..ticks {
            self.run_tick();
        }
    }
    pub fn tick(&self) -> usize {
        self.tick
    }
}

#[cfg(test)]
mod test {
    use crate::lang::parse::blocks::{neighborhood::{Dimension, NeighborhoodRule}, rule::{RuleASTNode, RuleTerminal, RulesBlock, TransitionRule}};

    use super::*;
    use super::DimensionBounds::*;
    use super::Coordinate::*;

    // Dimensions

    #[test]
    fn iterating_dimensions_1d() {
        let dims = DimensionBounds1D { x: (-3, 3) };
        assert_eq!(
            dims.into_iter().collect::<Vec<Coordinate>>(),
            vec!(-3, -2, -1, 0, 1, 2, 3).into_iter().map(|x| Coordinate1D {x}).collect::<Vec<Coordinate>>()
        );
    }

    #[test]
    fn iterating_dimensions_2d() {
        let dims = DimensionBounds2D { x: (-2, 2), y: (-2, 2) };
        assert_eq!(
            dims.into_iter().collect::<Vec<Coordinate>>(),
            vec!(
                (-2, -2), (-2, -1), (-2, 0), (-2, 1), (-2, 2),
                (-1, -2), (-1, -1), (-1, 0), (-1, 1), (-1, 2),
                (0, -2), (0, -1), (0, 0), (0, 1), (0, 2),
                (1, -2), (1, -1), (1, 0), (1, 1), (1, 2),
                (2, -2), (2, -1), (2, 0), (2, 1), (2, 2),
            ).into_iter().map(|(x, y)| Coordinate2D {x, y}).collect::<Vec<Coordinate>>()
        );
    }

    #[test]
    fn iterating_dimensions_3d() {
        let dims = DimensionBounds3D { x: (-2, 2), y: (-2, 2), z: (-2, 2) };
        assert_eq!(
            dims.into_iter().collect::<Vec<Coordinate>>(),
            vec!(
                (-2, -2, -2), (-2, -2, -1), (-2, -2, 0), (-2, -2, 1), (-2, -2, 2),
                (-2, -1, -2), (-2, -1, -1), (-2, -1, 0), (-2, -1, 1), (-2, -1, 2),
                (-2, 0, -2), (-2, 0, -1), (-2, 0, 0), (-2, 0, 1), (-2, 0, 2),
                (-2, 1, -2), (-2, 1, -1), (-2, 1, 0), (-2, 1, 1), (-2, 1, 2),
                (-2, 2, -2), (-2, 2, -1), (-2, 2, 0), (-2, 2, 1), (-2, 2, 2),

                (-1, -2, -2), (-1, -2, -1), (-1, -2, 0), (-1, -2, 1), (-1, -2, 2),
                (-1, -1, -2), (-1, -1, -1), (-1, -1, 0), (-1, -1, 1), (-1, -1, 2),
                (-1, 0, -2), (-1, 0, -1), (-1, 0, 0), (-1, 0, 1), (-1, 0, 2),
                (-1, 1, -2), (-1, 1, -1), (-1, 1, 0), (-1, 1, 1), (-1, 1, 2),
                (-1, 2, -2), (-1, 2, -1), (-1, 2, 0), (-1, 2, 1), (-1, 2, 2),

                (0, -2, -2), (0, -2, -1), (0, -2, 0), (0, -2, 1), (0, -2, 2),
                (0, -1, -2), (0, -1, -1), (0, -1, 0), (0, -1, 1), (0, -1, 2),
                (0, 0, -2), (0, 0, -1), (0, 0, 0), (0, 0, 1), (0, 0, 2),
                (0, 1, -2), (0, 1, -1), (0, 1, 0), (0, 1, 1), (0, 1, 2),
                (0, 2, -2), (0, 2, -1), (0, 2, 0), (0, 2, 1), (0, 2, 2),

                (1, -2, -2), (1, -2, -1), (1, -2, 0), (1, -2, 1), (1, -2, 2),
                (1, -1, -2), (1, -1, -1), (1, -1, 0), (1, -1, 1), (1, -1, 2),
                (1, 0, -2), (1, 0, -1), (1, 0, 0), (1, 0, 1), (1, 0, 2),
                (1, 1, -2), (1, 1, -1), (1, 1, 0), (1, 1, 1), (1, 1, 2),
                (1, 2, -2), (1, 2, -1), (1, 2, 0), (1, 2, 1), (1, 2, 2),

                (2, -2, -2), (2, -2, -1), (2, -2, 0), (2, -2, 1), (2, -2, 2),
                (2, -1, -2), (2, -1, -1), (2, -1, 0), (2, -1, 1), (2, -1, 2),
                (2, 0, -2), (2, 0, -1), (2, 0, 0), (2, 0, 1), (2, 0, 2),
                (2, 1, -2), (2, 1, -1), (2, 1, 0), (2, 1, 1), (2, 1, 2),
                (2, 2, -2), (2, 2, -1), (2, 2, 0), (2, 2, 1), (2, 2, 2),

            ).into_iter().map(|(x, y, z)| Coordinate3D {x, y, z}).collect::<Vec<Coordinate>>()
        );
    }

    #[test]
    fn contains_1d_true() {
        let dims = DimensionBounds::DimensionBounds1D {x: (-1, 1)};
        for x in (-1..=1) {
            assert!(dims.contains(Coordinate1D {x}))
        }
    }

    #[test]
    fn contains_2d_true() {
        let dims = DimensionBounds::DimensionBounds2D {x: (-1, 1), y: (-2, 2)};
        for x in (-1..=1) {
            for y in (-2..=2) {
                assert!(dims.contains(Coordinate2D {x, y}))
            }
        }
    }

    #[test]
    fn contains_3d_true() {
        let dims = DimensionBounds::DimensionBounds3D {x: (-1, 1), y: (-2, 2), z: (-3, 3)};
        for x in (-1..=1) {
            for y in (-2..=2) {
                for z in (-3..=3) {
                    assert!(dims.contains(Coordinate3D { x, y, z }))
                }
            }
        }
    }

    #[test]
    fn contains_1d_false() {
        let dims = DimensionBounds::DimensionBounds1D {x: (-10, 10)};
        assert!(!dims.contains(Coordinate1D {x: -11}));
        assert!(!dims.contains(Coordinate1D {x: 11}));
    }

    #[test]
    fn contains_2d_false() {
        let dims = DimensionBounds::DimensionBounds2D {x: (-1, 1), y: (-2, 2)};
        for x in (-1..1) {
            assert!(!dims.contains(Coordinate2D { x, y: 3 }));
            assert!(!dims.contains(Coordinate2D { x, y: -3 }));
        }
        for y in (-2..2) {
            assert!(!dims.contains(Coordinate2D { x: -2, y }));
            assert!(!dims.contains(Coordinate2D { x: 2, y }));
        }
    }

    #[test]
    fn contains_3d_false() {
        let dims = DimensionBounds::DimensionBounds3D {x: (-1, 1), y: (-2, 2), z: (-3, 3)};
        for x in (-1..1) {
            assert!(!dims.contains(Coordinate3D { x, y: 3, z: 0 }));
            assert!(!dims.contains(Coordinate3D { x, y: -3, z: 0 }));
            assert!(!dims.contains(Coordinate3D { x, y: 0, z: 4 }));
            assert!(!dims.contains(Coordinate3D { x, y: 0, z: -4 }));
        }
        for y in (-2..2) {
            assert!(!dims.contains(Coordinate3D { x: -2, y, z: 0 }));
            assert!(!dims.contains(Coordinate3D { x: 2, y, z: 0 }));
            assert!(!dims.contains(Coordinate3D { x: 0, y, z: 4 }));
            assert!(!dims.contains(Coordinate3D { x: 0, y, z: -4 }));
        }
        for z in (-3..3) {
            assert!(!dims.contains(Coordinate3D { x: -2, y: 0, z }));
            assert!(!dims.contains(Coordinate3D { x: 2, y: 0, z }));
            assert!(!dims.contains(Coordinate3D { x: 0, y: 3, z }));
            assert!(!dims.contains(Coordinate3D { x: 0, y: -3, z }));
        }
    }

    #[test]
    fn boundary_1d_true() {
        let dims = DimensionBounds::DimensionBounds1D {x: (-1, 1)};
        assert!(dims.boundary(Coordinate1D { x: 1 }));
        assert!(dims.boundary(Coordinate1D { x: -1 }));
    }
   
    #[test]
    fn boundary_2d_true() {
        let dims = DimensionBounds::DimensionBounds2D {x: (-1, 1), y: (-2, 2)};
        for x in (-1..=1) {
            assert!(dims.boundary(Coordinate2D { x, y: 2}));
            assert!(dims.boundary(Coordinate2D { x, y: -2}));
        }
        
        for y in (-2..=2) {
            assert!(dims.boundary(Coordinate2D { x: -1, y}));
            assert!(dims.boundary(Coordinate2D { x: 1, y}));
        }
    }

    #[test]
    fn boundary_3d_true() {
        let dims = DimensionBounds::DimensionBounds3D {x: (-1, 1), y: (-2, 2), z: (-3, 3)};
        for x in (-1..=1) {
            for y in (-2..=2) {
                assert!(dims.boundary(Coordinate3D { x, y, z: 3}));
                assert!(dims.boundary(Coordinate3D { x, y, z: -3}));
            }
        }
        for y in (-2..=2) {
            for z in (-3..=3) {
                assert!(dims.boundary(Coordinate3D { x: 1, y, z}));
                assert!(dims.boundary(Coordinate3D { x: -1, y, z}));
            }
        } 
        for x in (-1..=1) {
            for z in (-3..=3) {
                assert!(dims.boundary(Coordinate3D { x, y: 2, z}));
                assert!(dims.boundary(Coordinate3D { x, y: -2, z}));
            }
        }
    }
    
    #[test]
    fn boundary_1d_false() {
        let dims = DimensionBounds1D { x: (-3, 3) };
        for x in (-2..=2) {
            assert!(!dims.boundary(Coordinate1D { x }));
        }
    }
    
    #[test]
    fn boundary_2d_false() {
        let dims = DimensionBounds::DimensionBounds2D { x: (-2, 2), y: (-2, 2) };
        for x in (-1..=1) {
            for y in (-1..=1) {
                assert!(!dims.boundary(Coordinate2D { x, y }))
            }
        }
    }
    
    #[test]
    fn boundary_3d_false() {
        let dims = DimensionBounds::DimensionBounds3D { x: (-2, 2), y: (-2, 2), z: (-2, 2) };
        for x in (-1..=1) {
            for y in (-1..=1) {
                for z in (-1..=1) {
                    assert!(!dims.boundary(Coordinate3D { x, y, z }))
                }
            }
        }
    }

    // Runtime

    #[test]
    fn runtime_oscillate_single_cell_1d() {
        let mut state_map = HashMap::new();
        state_map.insert("A".into(), 0);
        state_map.insert("B".into(), 1);

        let mut rt = Runtime::new(
            DimensionBounds1D { x: (-0, 0)},
            BoundaryBlock::Void,
            States::new(
                2,
                state_map.clone(),
                HashMap::new(),
                None,
            ),
            Rules::new(
                RulesBlock::new(
                    vec!(
                        TransitionRule {
                            from: "A".into(),
                            to: "B".into(),
                            root: Box::new(RuleASTNode::GreaterThan {
                                lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(2))),
                                rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1))),
                            })
                        }
                    )
                ),
                &state_map
            ),
            Neighborhood::new(vec!())
        );
        rt.set_cell(Coordinate1D { x: 0 }, 0);
        assert_eq!(rt.get_state(Coordinate1D { x: 0 }), Some(0));
        rt.run_tick();
        assert_eq!(rt.get_state(Coordinate1D { x: 0 }), Some(1));
    }
    
    #[test]
    fn runtime_propogate_infinite_boundary_1d() {
        let mut state_map = HashMap::new();
        state_map.insert("A".into(), 0);
        state_map.insert("B".into(), 1);

        let mut rt = Runtime::new(
            DimensionBounds1D { x: (-5, 5) },
            BoundaryBlock::Infinite,
            States::new(
                2,
                state_map.clone(),
                HashMap::new(),
                Some(0),
            ),
            Rules::new(
                RulesBlock::new(
                    vec!(
                        TransitionRule {
                            from: "A".into(),
                            to: "B".into(),
                            root: Box::new(RuleASTNode::GreaterThanOrEqualTo {
                                lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1))),
                            })
                        }
                    )
                ),
                &state_map
            ),
            Neighborhood::new(vec!(
                NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1 },
            ))
        );
        rt.set_cell(Coordinate1D { x: 0}, 1);

        for t in (1..10) {
            assert_eq!(rt.get_state(Coordinate1D { x: t }), Some(0));
            assert_eq!(rt.get_state(Coordinate1D { x: -t }), Some(0));
            rt.run_tick();
            for x in (0..t) {
                assert_eq!(rt.get_state(Coordinate1D { x }), Some(1));
                assert_eq!(rt.get_state(Coordinate1D { x: -x }), Some(1));
            }
        }
    }
    
    #[test]
    fn runtime_does_not_alter_static_boundary_cells_1d() {
        let mut state_map = HashMap::new();
        state_map.insert("A".into(), 0);
        state_map.insert("B".into(), 1);

        let mut rt = Runtime::new(
            DimensionBounds1D { x: (-1, 1) },
            BoundaryBlock::Static(Some("A".into())),
            States::new(
                2,
                state_map.clone(),
                HashMap::new(),
                Some(0),
            ),
            Rules::new(
                RulesBlock::new(
                    vec!(
                        TransitionRule {
                            from: "A".into(),
                            to: "B".into(),
                            root: Box::new(RuleASTNode::GreaterThanOrEqualTo {
                                lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("B".into()))),
                                rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1))),
                            })
                        }
                    )
                ),
                &state_map
            ),
            Neighborhood::new(vec!(
                NeighborhoodRule::UndirectedEdge { dimension: Dimension::X, magnitude: 1 },
            ))
        );
        rt.set_cell(Coordinate1D { x: 0}, 1);

        assert_eq!(rt.get_state(Coordinate1D { x: 1 }), Some(0));
        assert_eq!(rt.get_state(Coordinate1D { x: -1 }), Some(0));
        rt.run_tick();
        assert_eq!(rt.get_state(Coordinate1D { x: 1 }), Some(0));
        assert_eq!(rt.get_state(Coordinate1D { x: -1 }), Some(0));
    }
}
