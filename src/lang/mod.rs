use crate::lang::parse::blocks::boundary::BoundaryBlock;
use crate::lang::parse::blocks::neighborhood::NeighborhoodBlock;
use crate::lang::parse::blocks::environment::EnvironmentBlock;
use crate::lang::parse::blocks::state::StatesBlock;
use crate::lang::parse::blocks::rule::RulesBlock;

pub mod parse;

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