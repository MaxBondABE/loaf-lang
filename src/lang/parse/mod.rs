use pest::Parser;
use pest::error::Error as PestError;
use pest_derive::Parser;
use pest::iterators::{Pair, Pairs};
use crate::lang::ProgramBuilder;
use std::convert::TryInto;
use std::num::ParseIntError;
use crate::lang::parse::blocks::boundary::BoundaryBlock;
use crate::lang::parse::blocks::neighborhood::NeighborhoodBlock;
use crate::lang::parse::blocks::environment::EnvironmentBlock;
use crate::lang::parse::blocks::state::StatesBlock;

pub mod blocks;

#[derive(Parser)]
#[grammar="lang/parse/loaf.pest"]
pub(crate) struct LoafParser;
pub(crate) type LoafPair<'a> = Pair<'a, Rule>;
pub(crate) type LoafPairs<'a> = Pairs<'a, Rule>;

pub fn parse(s: &str) -> Result<ProgramBuilder, Error> {
    let root = LoafParser::parse(Rule::program, s)?;
    let mut builder = ProgramBuilder::new();

    let mut parsed_boundary = false;
    let mut parsed_neighborhood = false;
    let mut parsed_environment = false;
    let mut parsed_states = false;
    let mut parsed_rules = false;
    for pair in root {
        match pair.as_rule() {
            Rule::boundary_block => {
                if parsed_boundary {
                    return Err(Error::MultipleDefinitionsForBlock);
                }
                parsed_boundary = true;
                builder.boundary(pair.try_into()?);
            },
            Rule::neighborhood_block => {
                if parsed_neighborhood {
                    return Err(Error::MultipleDefinitionsForBlock);
                }
                parsed_neighborhood = true;
                builder.neighborhood(pair.try_into()?);
            },
            Rule::environment_block => {
                if parsed_environment {
                    return Err(Error::MultipleDefinitionsForBlock);
                }
                parsed_environment = true;
                builder.environment(pair.try_into()?);
            },
            Rule::state_block => {
                if parsed_states {
                    return Err(Error::MultipleDefinitionsForBlock);
                }
                parsed_states = true;
                builder.states(pair.try_into()?);
            }
            Rule::rule_block => {
                if parsed_rules {
                    return Err(Error::MultipleDefinitionsForBlock);
                }
                parsed_rules = true;
                builder.rules(pair.try_into()?);
            }
            Rule::EOI => break,
            _ => unreachable!()
        }
    }
    Ok(builder)
}

#[derive(Debug)]
pub enum Error {
    SyntaxError(PestError<Rule>),
    UnrepresentableNumber(ParseIntError),
    MultipleDefinitionsForBlock, // TODO include pair - triggers lifetime issues
    MultipleDefaultStates
}
impl From<PestError<Rule>> for Error {
    fn from(error: PestError<Rule>) -> Self { Self::SyntaxError(error) }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self { Self::UnrepresentableNumber(error) }
}
