use std::collections::HashMap;

use runtime::{datatypes::coords::DimensionBounds, Runtime, naive::NaiveRuntime};

use crate::{lang::parse::blocks::boundary::BoundaryBlock, render::{Output, render2d::Render2D}};
use crate::lang::parse::blocks::neighborhood::NeighborhoodBlock;
use crate::lang::parse::blocks::environment::EnvironmentBlock;
use crate::lang::parse::blocks::state::StatesBlock;
use crate::lang::parse::blocks::rule::RulesBlock;

use self::runtime::{datatypes::{coords::Coordinate, states::States}, ops::{neighborhood::Neighborhood, rules::Rules}};

pub mod parse;
pub mod runtime;

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
    pub fn build(self) -> Program {
        let states = States::from_block(self.states.unwrap());
        let names_map = states.name_map();
        let color_map = states.color_map();
        let dim_bounds = self.environment.unwrap().dimensions();
        Program {
            runtime: Box::new(NaiveRuntime::new(
                dim_bounds,
                self.boundary.unwrap(),
                states,
                Rules::from_block(self.rules.unwrap(), &names_map),
                Neighborhood::from_block(self.neighborhood.unwrap())
            )),
            // TODO parameterize default color, name, cell width
            output: Box::new(Render2D::new(
                        color_map,
                        dim_bounds,
                        image::Rgb([0xff,0xff, 0xff]),
                        "Simulation".into(),
                        50
                    ))
        }
    }

    // Validation
    fn validate(&mut self) -> bool {
        unimplemented!()
    }
    fn warnings(&self) -> Vec<Warnings> {
        unimplemented!()
    }
}

pub struct Program {
    runtime: Box<dyn Runtime>,
    output: Box<dyn Output>
}

impl Program {
    pub fn run(&mut self, ticks: usize) {
        self.runtime.set_env({
            let mut h = HashMap::new();
            h.insert(Coordinate::Coordinate2D { x: 0, y: 0 }, 1);
            //h.insert(Coordinate::Coordinate2D { x: 0, y: 1}, 1);
            //h.insert(Coordinate::Coordinate2D { x: 0, y: -1}, 1);
            //h.insert(Coordinate::Coordinate2D { x: 1, y: 1}, 1);
            h
        });
        let env = self.runtime.get_env();
        //self.output.output_tick(env.into_iter().collect());
        for t in (0..ticks) {
            eprintln!("Before tick {} {:?}", t, self.runtime.get_env());
            //self.output.output_tick(
            self.runtime.run_tick();
            //   );
            eprintln!("After tick {} {:?}", t, self.runtime.get_env());
        }
        eprintln!("{:?}", self.runtime.get_env());
    }
}

pub enum Warnings {
    UnknownColor
}
