use crate::lang::parse::{LoafPair, Rule, Error as ParseError};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

const NUM_COLORS:usize = 7;
const BUILTIN_COLORS: [(&str, (u8, u8, u8)); NUM_COLORS] = [
    ("black", (0, 0, 0)),
    ("white", (255, 255, 255)),
    ("grey", (0xf0, 0xf0, 0xf0)),
    ("gray", (0xf0, 0xf0, 0xf0)),
    ("red", (255, 0, 0)),
    ("green", (0, 255, 0)),
    ("blue", (0, 0, 255)),
];

#[derive(Debug, Eq, PartialEq)]
pub struct StatesBlock {
    states: HashMap<String, Vec<Attribute>>
}
impl TryFrom<LoafPair<'_>> for StatesBlock {
    type Error = ParseError;

    fn try_from(pair: LoafPair<'_>) -> Result<Self, Self::Error> {
        debug_assert_eq!(pair.as_rule(), Rule::state_block);
        let mut block = HashMap::new();
        let mut found_default = false;
        for state in pair.into_inner() {
            if state.as_rule() == Rule::EOI {
                // Annoying hack because I can't seem to silence EOI
                break;
            }
            let mut children = state.into_inner();
            let name = children.next().expect("States have at least 1 child.").as_str().into();
            let mut attributes = Vec::new();
            for attribute in children {
                attributes.push(attribute.try_into()?);
            }
            block.insert(name, attributes);
        }
        Ok(Self { states: block })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Attribute {
    Default,
    Color(Option<(u8, u8, u8)>), // None for unknown colors - which is a warning, not an error
    //Other(String, Option(String)) // For future features, plugins, alternative renderers, etc
}
impl TryFrom<LoafPair<'_>> for Attribute {
    type Error = ParseError;

    fn try_from(pair: LoafPair<'_>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::default_attribute => Ok(Self::Default),
            Rule::color_attribute => Ok(Self::Color(parse_color(pair))),
            _ => unimplemented!()
        }
    }
}

fn parse_color(pair: LoafPair<'_>) -> Option<(u8, u8, u8)> {
    let child = pair.into_inner().next().expect("Color attribute has exactly 1 child.");
    match child.as_rule() {
        Rule::rgb => {
            let s = child.as_str();
            let r = &s[1..=2];
            let g = &s[3..=4];
            let b = &s[5..=6];

            Some((
                u8::from_str(r).expect("RGB values guaranteed to fit in byte."),
                u8::from_str(g).expect("RGB values guaranteed to fit in byte."),
                u8::from_str(b).expect("RGB values guaranteed to fit in byte.")
            ))
        },
        Rule::name => {
            BUILTIN_COLORS.iter()
                    .find(|(name, _)| *name == child.as_str()).map(|(_, rgb)| *rgb)
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lang::parse::LoafParser;
    use pest::Parser;
    use std::convert::TryInto;

    #[test]
    fn state_without_attributes() {
        let state = LoafParser::parse(Rule::state_block, "state := { A }");
        assert!(state.is_ok()); // Parsed successfully
        let state: Result<StatesBlock, _> = state.unwrap().next().unwrap().try_into();
        assert!(state.is_ok()); // Converted successfully
        assert_eq!(state.unwrap(),  {
            let mut states = HashMap::new();
            states.insert("A".into(), vec!());
            StatesBlock { states }
        });
    }

    #[test]
    fn state_with_default() {
        let state = LoafParser::parse(Rule::state_block, "state := { A::(default) }");
        assert!(state.is_ok()); // Parsed successfully
        let state: Result<StatesBlock, _> = state.unwrap().next().unwrap().try_into();
        assert!(state.is_ok()); // Converted successfully
        assert_eq!(state.unwrap(),  {
            let mut states = HashMap::new();
            states.insert("A".into(), vec!(Attribute::Default));
            StatesBlock { states }
        });
    }

    #[test]
    fn state_with_named_color() {
        let state = LoafParser::parse(Rule::state_block, "state := { A::(color=\"white\") }");
        assert!(state.is_ok()); // Parsed successfully
        let state: Result<StatesBlock, _> = state.unwrap().next().unwrap().try_into();
        assert!(state.is_ok()); // Converted successfully
        assert_eq!(state.unwrap(),  {
            let mut states = HashMap::new();
            states.insert("A".into(), vec!(Attribute::Color(Some((255, 255, 255)))));
            StatesBlock { states }
        });
    }

    #[test]
    fn state_with_hex_color() {
        let state = LoafParser::parse(Rule::state_block, "state := { A::(color=#010203) }");
        assert!(state.is_ok()); // Parsed successfully
        let state: Result<StatesBlock, _> = state.unwrap().next().unwrap().try_into();
        assert!(state.is_ok()); // Converted successfully
        assert_eq!(state.unwrap(),  {
            let mut states = HashMap::new();
            states.insert("A".into(), vec!(Attribute::Color(Some((1, 2, 3)))));
            StatesBlock { states }
        });
    }

    #[test]
    fn state_with_color_and_default() {
        let state = LoafParser::parse(Rule::state_block, "state := { A::(color=#010203, default) }");
        assert!(state.is_ok()); // Parsed successfully
        let state: Result<StatesBlock, _> = state.unwrap().next().unwrap().try_into();
        assert!(state.is_ok()); // Converted successfully
        assert_eq!(state.unwrap(),  {
            let mut states = HashMap::new();
            states.insert("A".into(), vec!(
                Attribute::Color(Some((1, 2, 3))),
                Attribute::Default
            ));
            StatesBlock { states }
        });
    }
}