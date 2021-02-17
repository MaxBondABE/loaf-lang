use std::collections::HashMap;

use loaf_lang::datatypes::coords::{BoundingBox2D, Coordinate, Coordinate2D, Dimension};
use loaf_lang::runtime::environment::naive::FixedGrid;
use loaf_lang::runtime::environment::Environment;
use loaf_lang::runtime::neighborhood::{Rule as NeighborhoodRule, Ruleset as NeighborhoodRuleset};
use loaf_lang::runtime::state::{
    ASTRoot, CensusNode, EqNode, GtNode, LoafType, LtNode, OrNode, Ruleset as StateRuleset,
};
use loaf_lang::runtime::{Runtime, SynchronousRuntime};

const DEAD: usize = 0;
const ALIVE: usize = 1;

pub fn conway_runtime(
    bounds: BoundingBox2D,
    initial_states: HashMap<Coordinate2D, usize>,
) -> SynchronousRuntime<usize, Vec<usize>, FixedGrid<Coordinate2D, BoundingBox2D>, Vec<Coordinate2D>>
{
    let revive: ASTRoot<usize, Vec<usize>> = ASTRoot::new(
        EqNode::new(
            CensusNode::new(ALIVE).boxed(),
            Box::new(LoafType::Integer(3)),
        )
        .boxed(),
    );
    let die: ASTRoot<usize, Vec<usize>> = ASTRoot::new(
        OrNode::new(
            LtNode::new(
                CensusNode::new(ALIVE).boxed(),
                Box::new(LoafType::Integer(2)),
            )
            .boxed(),
            GtNode::new(
                CensusNode::new(ALIVE).boxed(),
                Box::new(LoafType::Integer(3)),
            )
            .boxed(),
        )
        .boxed(),
    );
    let state_rules: StateRuleset<usize, Vec<usize>> =
        StateRuleset::new(vec![(DEAD, (revive, ALIVE)), (ALIVE, (die, DEAD))]);
    let neighborhood_rules: NeighborhoodRuleset<Coordinate2D> = NeighborhoodRuleset::new(vec![
        NeighborhoodRule::undirected_edge(Dimension::All, 1),
        NeighborhoodRule::compound_rule(
            NeighborhoodRule::undirected_edge(Dimension::X, 1),
            NeighborhoodRule::undirected_edge(Dimension::Y, 1),
        ),
    ]);
    let neighborhood: Vec<Coordinate2D> = neighborhood_rules.into_iter().collect();

    let env = FixedGrid::from_hashmap(neighborhood.into_boxed_slice(), initial_states, bounds);
    let runtime = SynchronousRuntime::new(state_rules, env);
    runtime
}

fn print_snapshot(snapshot: HashMap<Coordinate2D, usize>) {
    let mut sorted = snapshot.keys().map(|c| *c).collect::<Vec<_>>();
    sorted.sort_by_key(|c| (-c.y(), c.x()));
    let mut last_y = sorted.first().unwrap().y();
    for coord in sorted {
        if coord.y() != last_y {
            println!("");
            last_y = coord.y();
        }
        if snapshot[&coord] == ALIVE {
            print!("[*]");
        } else {
            print!("[ ]");
        }
    }
    println!("");
}

#[cfg(test)]
pub mod conway_integration_tests {
    use super::*;

    #[test]
    fn test_trivial_block_still_life() {
        let bounds = BoundingBox2D::new((-1, 2), (-1, 2));
        let initial_states = vec![
            (Coordinate2D::new(0, 0), ALIVE),
            (Coordinate2D::new(1, 0), ALIVE),
            (Coordinate2D::new(1, 1), ALIVE),
            (Coordinate2D::new(0, 1), ALIVE),
        ]
        .into_iter()
        .collect();
        let mut rt = conway_runtime(bounds, initial_states);
        let before = rt.environment().snapshot();

        println!("Tick 0");
        print_snapshot(rt.environment().snapshot());
        assert!(rt.run_tick().is_empty());
        println!("Tick 1");
        print_snapshot(rt.environment().snapshot());
        assert_eq!(rt.environment().snapshot(), before);
    }

    #[test]
    fn test_trivial_blinker_oscillator() {
        let bounds = BoundingBox2D::new((-2, 2), (-2, 2));
        let initial_states = vec![
            (Coordinate2D::new(1, 0), ALIVE),
            (Coordinate2D::new(0, 0), ALIVE),
            (Coordinate2D::new(-1, 0), ALIVE),
        ]
        .into_iter()
        .collect();
        let mut rt = conway_runtime(bounds, initial_states);
        let before = rt.environment().snapshot();

        println!("Tick 0");
        print_snapshot(rt.environment().snapshot());
        assert_eq!(
            rt.run_tick(),
            vec!(
                (Coordinate2D::new(1, 0), DEAD),
                (Coordinate2D::new(-1, 0), DEAD),
                (Coordinate2D::new(0, 1), ALIVE),
                (Coordinate2D::new(0, -1), ALIVE),
            )
            .into_iter()
            .collect()
        );

        println!("Tick 1");
        print_snapshot(rt.environment().snapshot());
        assert_eq!(
            rt.run_tick(),
            vec!(
                (Coordinate2D::new(1, 0), ALIVE),
                (Coordinate2D::new(-1, 0), ALIVE),
                (Coordinate2D::new(0, 1), DEAD),
                (Coordinate2D::new(0, -1), DEAD),
            )
            .into_iter()
            .collect()
        );

        assert_eq!(rt.environment().snapshot(), before);
    }
}
