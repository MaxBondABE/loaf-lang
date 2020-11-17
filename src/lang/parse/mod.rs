use pest::Parser;
use pest::error::Error as PestError;
use pest_derive::Parser;
use pest::iterators::{Pair, Pairs};
use crate::lang::ProgramBuilder;
use std::convert::TryInto;
use std::num::ParseIntError;

pub mod blocks;

#[derive(Parser)]
#[grammar="lang/parse/loaf.pest"]
pub(crate) struct LoafParser;
pub(crate) type LoafPair<'a> = Pair<'a, Rule>;
pub(crate) type LoafPairs<'a> = Pairs<'a, Rule>;

pub fn parse(s: &str) -> Result<ProgramBuilder, Error> {
    let root = LoafParser::parse(Rule::program, s)?;

    let mut boundary_pair: Option<LoafPair> = None;
    let mut neighborhood_pair: Option<LoafPair> = None;
    let mut environment_pair: Option<LoafPair> = None;
    let mut state_pair: Option<LoafPair> = None;
    let mut rule_pair: Option<LoafPair> = None;
    for pair in root {
        match pair.as_rule() {
            Rule::boundary_block => {
                boundary_pair = Some(pair)
            },
            Rule::neighborhood_block => {
                neighborhood_pair = Some(pair)
            },
            Rule::environment_block => {
                environment_pair = Some(pair)
            },
            Rule::state_block => {
                state_pair = Some(pair)
            }
            Rule::rule_block => {
                rule_pair = Some(pair)
            }
            Rule::EOI => break,
            _ => unreachable!()
        }
    }
    Ok(
        ProgramBuilder::new(
            boundary_pair.unwrap().try_into()?,
            neighborhood_pair.unwrap().try_into()?,
            environment_pair.unwrap().try_into()?,
            state_pair.unwrap().try_into()?,
            rule_pair.unwrap().try_into()?
        )
    )
}

#[derive(Debug)]
pub enum Error {
    SyntaxError(PestError<Rule>),
    UnrepresentableNumber(ParseIntError),
    MultipleDefaultStates
}
impl From<PestError<Rule>> for Error {
    fn from(error: PestError<Rule>) -> Self { Self::SyntaxError(error) }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self { Self::UnrepresentableNumber(error) }
}
