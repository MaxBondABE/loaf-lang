use crate::lang::parse::blocks::boundary::BoundaryBlock;
use crate::lang::parse::blocks::neighborhood::NeighborhoodBlock;
use crate::lang::parse::blocks::environment::EnvironmentBlock;
use crate::lang::parse::blocks::state::StatesBlock;
use crate::lang::parse::blocks::rule::RulesBlock;

pub mod parse;

#[derive(Debug)]
pub struct ProgramBuilder {
    boundary: Option<BoundaryBlock>,
    environment: Option<EnvironmentBlock>,
    neighborhood: Option<NeighborhoodBlock>,
    states: Option<StatesBlock>,
    rules: Option<RulesBlock>,
    valid: Option<bool>
}
impl Default for ProgramBuilder {
    fn default() -> Self {
        Self {
            boundary: None,
            neighborhood: None,
            environment: None,
            states: None,
            rules: None,
            valid: None
        }
    }
}
impl ProgramBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder methods
    pub fn boundary(&mut self, b: BoundaryBlock) -> &mut Self {
        self.boundary = Some(b);
        self
    }
    pub fn environment(&mut self, e: EnvironmentBlock) -> &mut Self {
        self.environment = Some(e);
        self
    }
    pub fn neighborhood(&mut self, n: NeighborhoodBlock) -> &mut Self {
        self.neighborhood = Some(n);
        self
    }
    pub fn states(&mut self, s: StatesBlock) -> &mut Self {
        self.states = Some(s);
        self
    }
    pub fn rules(&mut self, r: RulesBlock) -> &mut Self {
        self.rules = Some(r);
        self
    }
    pub fn build(&self) {
        unimplemented!()
    }

    // Validation
    fn validate(&mut self) -> bool {
        unimplemented!()
    }
    fn warnings(&self) -> Vec<Warnings> {
        unimplemented!()
    }
}

pub enum Warnings {
    UnknownColor
}