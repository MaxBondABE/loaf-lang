use crate::lang::blocks::boundary::BoundaryBlock;
use crate::lang::blocks::neighborhood::NeighborhoodBlock;
use crate::lang::blocks::environment::EnvironmentBlock;
use crate::lang::blocks::state::StatesBlock;
use crate::lang::blocks::rule::RulesBlock;

pub mod parse;
pub mod blocks;

#[derive(Debug)]
pub struct ProgramBuilder {
    boundary: BoundaryBlock,
    neighborhood: NeighborhoodBlock,
    environment: EnvironmentBlock,
    state: StatesBlock,
    rule: RulesBlock
}
impl ProgramBuilder {
    pub fn new(boundary: BoundaryBlock, neighborhood: NeighborhoodBlock, environment: EnvironmentBlock, state: StatesBlock, rule: RulesBlock) -> Self {
        Self {
            boundary,
            neighborhood,
            environment,
            state,
            rule
        }
    }
}