use std::convert::{TryFrom, TryInto};
use crate::lang::parse::{LoafPair, Rule, Error as ParseError};
use std::str::FromStr;

// TODO compound rules

#[derive(Debug, Eq, PartialEq)]
pub enum NeighborhoodBlock {
    Moore,
    VonNeumann,
    Custom(Vec<NeighborhoodRule>)
}
impl TryFrom<LoafPair<'_>> for NeighborhoodBlock {
    type Error = ParseError;

    fn try_from(pair: LoafPair<'_>) -> Result<Self, Self::Error> {
        debug_assert_eq!(pair.as_rule(), Rule::neighborhood_block);

        let pair = pair.into_inner()
            .next()
            .expect("Neighborhood block grammar contains exactly 1 child.");

        match pair.as_rule()
        {
            Rule::moore_neighborhood => Ok(Self::Moore),
            Rule::von_neumann_neighborhood => Ok(Self::VonNeumann),
            Rule::neighborhood_rules => {
                let mut rules = Vec::new();
                for result in pair.into_inner().into_iter().map(
                    |p| { let o: Result<NeighborhoodRule, _> = p.try_into(); o }
                ) {
                    rules.push(result?);
                }
                Ok(Self::Custom(rules))
            },
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum NeighborhoodRule {
    UndirectedEdge {dimension: Dimension, magnitude: usize},
    DirectedEdge { dimension: Dimension, magnitude: usize, direction: EdgeDirection},
    UndirectedCircle {dimension: Dimension, magnitude: usize}
}
impl TryFrom<LoafPair<'_>> for NeighborhoodRule {
    type Error = ParseError;

    fn try_from(pair: LoafPair<'_>) -> Result<Self, Self::Error> {
        let rule = pair.as_rule();
        let mut children = pair.into_inner();
        let dimension: Dimension = children
            .next().expect("Neighborhood rules should have exactly 2 children.").try_into()?;
        let magnitude = usize::from_str(
            children
                .next().expect("Neighborhood rules should have exactly 2 children.").as_str()
        )?;
        match rule {
            Rule::directed_positive => Ok(Self::DirectedEdge
                {dimension, magnitude, direction: EdgeDirection::Positive}),
            Rule::directed_negative => Ok(Self::DirectedEdge
                {dimension, magnitude, direction: EdgeDirection::Negative}),
            Rule::undirected_edge => Ok(Self::UndirectedEdge {dimension, magnitude}),
            Rule::undirected_circle => Ok(Self::UndirectedCircle {dimension, magnitude}),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Dimension {
    X,
    Y,
    Z,
    All
}
// TODO From instead? Shouldn't fail, panics instead of error anyway..
impl TryFrom<LoafPair<'_>> for Dimension {
    type Error = ParseError;

    fn try_from(pair: LoafPair<'_>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::x_dimension => Ok(Self::X),
            Rule::y_dimension => Ok(Self::Y),
            Rule::z_dimension => Ok(Self::Z),
            Rule::all_dimensions => Ok(Self::All),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum EdgeDirection {
    Positive,
    Negative
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lang::parse::LoafParser;
    use pest::Parser;
    use std::convert::TryInto;

    #[test]
    fn moore() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := MOORE");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Moore)
    }

    #[test]
    fn von_neumann() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := VON_NEUMANN");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::VonNeumann)
    }

    #[test]
    fn custom_single_dim_directed_pos() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := { x + 1 }");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Custom(vec!(
            NeighborhoodRule::DirectedEdge {
                dimension: Dimension::X,
                magnitude: 1,
                direction: EdgeDirection::Positive
            }
        )))
    }

    #[test]
    fn custom_single_dim_directed_neg() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := { x - 1 }");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Custom(vec!(
            NeighborhoodRule::DirectedEdge {
                dimension: Dimension::X,
                magnitude: 1,
                direction: EdgeDirection::Negative
            }
        )))
    }

    #[test]
    fn custom_all_dims_directed_pos() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := { * + 1 }");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Custom(vec!(
            NeighborhoodRule::DirectedEdge {
                dimension: Dimension::All,
                magnitude: 1,
                direction: EdgeDirection::Positive
            }
        )))
    }

    #[test]
    fn custom_all_dims_directed_neg() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := { * - 1 }");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Custom(vec!(
            NeighborhoodRule::DirectedEdge {
                dimension: Dimension::All,
                magnitude: 1,
                direction: EdgeDirection::Negative
            }
        )))
    }


    #[test]
    fn custom_single_dim_undirected() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := { x +- 1 }");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Custom(vec!(
            NeighborhoodRule::UndirectedEdge {
                dimension: Dimension::X,
                magnitude: 1,
            }
        )))
    }

    #[test]
    fn custom_all_dims_undirected() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := { * +- 1 }");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Custom(vec!(
            NeighborhoodRule::UndirectedEdge {
                dimension: Dimension::All,
                magnitude: 1,
            }
        )))
    }

    #[test]
    fn custom_single_dim_circle() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := { x within 1 }");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Custom(vec!(
            NeighborhoodRule::UndirectedCircle {
                dimension: Dimension::X,
                magnitude: 1,
            }
        )))
    }

    #[test]
    fn custom_all_dims_circle() {
        let nb = LoafParser::parse(Rule::neighborhood_block, "neighborhood := { * within 1 }");
        assert!(nb.is_ok()); // Parsed successfully
        let nb: Result<NeighborhoodBlock,_> = nb.unwrap().next().unwrap().try_into();
        assert!(nb.is_ok()); // Converted successfully
        assert_eq!(nb.unwrap(), NeighborhoodBlock::Custom(vec!(
            NeighborhoodRule::UndirectedCircle {
                dimension: Dimension::All,
                magnitude: 1,
            }
        )))
    }
}