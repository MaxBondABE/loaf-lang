#[macro_use]
use lazy_static::lazy_static;
use pest::prec_climber::{PrecClimber, Operator, Assoc};

use crate::lang::parse::{Rule, LoafPair, LoafPairs, Error as ParseError};
use pest::iterators::Pairs;
use std::str::FromStr;
use std::convert::{TryFrom, TryInto};

lazy_static!(
    static ref PRECEDENCE_CLIMBER: PrecClimber<Rule> = {
        PrecClimber::new(vec!(
            Operator::new(Rule::and, Assoc::Left) | Operator::new(Rule::or, Assoc::Left),
            Operator::new(Rule::equal, Assoc::Left) | Operator::new(Rule::not_equal, Assoc::Left),
            Operator::new(Rule::gt, Assoc::Left) | Operator::new(Rule::gte, Assoc::Left) |
                Operator::new(Rule::lt, Assoc::Left) | Operator::new(Rule::lte, Assoc::Left),
            Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Left),
            Operator::new(Rule::mul, Assoc::Left) | Operator::new(Rule::div, Assoc::Left)
        ))
    };
);

fn build_ast(expression: LoafPairs) -> Box<RuleASTNode> {
    PRECEDENCE_CLIMBER.climb(
        expression,
        |pair: LoafPair<'_>| match pair.as_rule() {
            Rule::integer => Box::new(RuleASTNode::Terminal(
                // TODO propogate error
                RuleTerminal::Number(isize::from_str(pair.as_str()).unwrap()))
            ),
            Rule::census => Box::new(RuleASTNode::Terminal({
                let name = pair.into_inner().next().expect("Census has exactly 1 child.");
                RuleTerminal::Census(name.as_str().into())
            })),
            Rule::rule_statement => build_ast(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: Box<RuleASTNode>, op: LoafPair<'_>, rhs: Box<RuleASTNode>| match op.as_rule() {
            Rule::plus      => Box::new(RuleASTNode::Add {lhs, rhs}),
            Rule::minus => Box::new(RuleASTNode::Sub {lhs, rhs}),
            Rule::mul => Box::new(RuleASTNode::Mul {lhs, rhs}),
            Rule::div   => Box::new(RuleASTNode::Div {lhs, rhs}),
            Rule::gt => Box::new(RuleASTNode::GreaterThan {lhs, rhs}),
            Rule::gte => Box::new(RuleASTNode::GreaterThanOrEqualTo {lhs, rhs}),
            Rule::lt => Box::new(RuleASTNode::LessThan {lhs, rhs}),
            Rule::lte => Box::new(RuleASTNode::LessThanOrEqualTo {lhs, rhs}),
            Rule::equal => Box::new(RuleASTNode::Equal {lhs, rhs}),
            Rule::not_equal => Box::new(RuleASTNode::NotEqual {lhs, rhs}),
            Rule::and => Box::new(RuleASTNode::And {lhs, rhs}),
            Rule::or => Box::new(RuleASTNode::Or {lhs, rhs}),
            _ => unreachable!(),
        },
    )
}

#[derive(Debug, Eq, PartialEq)]
pub struct RulesBlock {
    rules: Vec<TransitionRule>
}
impl TryFrom<LoafPair<'_>> for RulesBlock {
    type Error = ParseError;

    fn try_from(pair: LoafPair<'_>) -> Result<Self, Self::Error> {
        let mut rules = Vec::new();
        for rule in pair.into_inner() {
            if rule.as_rule() == Rule::EOI {
                // Annoying hack because I can't seem to silence EOI
                break;
            }
            let r: TransitionRule = rule.try_into()?;
            rules.push(r);
        }
        Ok(Self { rules })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct TransitionRule {
    pub from: String,
    pub to: String,
    pub root: Box<RuleASTNode>
}
impl TryFrom<LoafPair<'_>> for TransitionRule {
    type Error = ParseError;

    fn try_from(pair: LoafPair<'_>) -> Result<Self, Self::Error> {
        let mut children = pair.into_inner();
        let from = children.next().expect("Rule statement has exactly 3 children.").as_str().into();
        let to = children.next().expect("Rule statement has exactly 3 children.").as_str().into();
        let root = build_ast(
            children.next().expect("Rule statement has exactly 3 children.").into_inner()
        );
        Ok(Self { from, to, root })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RuleASTNode {
    Terminal(RuleTerminal),
    Add { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    Sub { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    Mul { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    Div { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    And { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    Or { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    GreaterThan { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    GreaterThanOrEqualTo { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    LessThan { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    LessThanOrEqualTo { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    Equal { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> },
    NotEqual { lhs: Box<RuleASTNode>, rhs: Box<RuleASTNode> }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RuleTerminal {
    Number(isize), // TODO is this the type?
    Census(String)
}

// TODO compound rules
#[cfg(test)]
mod test {
    use super::*;
    use crate::lang::parse::LoafParser;
    use pest::Parser;
    use std::convert::TryInto;

    #[test]
    fn simple_eq() {
        let rules = LoafParser::parse(Rule::rule_block,
                                      "rule := { from A to B :=  neighborhood(A) = 1 }");
        assert!(rules.is_ok()); // Parsed successfully
        let rules: Result<RulesBlock, _> = rules.unwrap().next().unwrap().try_into();
        assert!(rules.is_ok()); // Converted successfully
        assert_eq!(rules.unwrap(),
                   RulesBlock { rules: vec!(
                       TransitionRule {
                           from: "A".into(),
                           to: "B".into(),
                           root: Box::new(
                               RuleASTNode::Equal {
                                   lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                   rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                               }
                           )
                       }
                   )}
        );
    }

    #[test]
    fn simple_neq() {
        let rules = LoafParser::parse(Rule::rule_block,
                                      "rule := { from A to B :=  neighborhood(A) != 1 }");
        assert!(rules.is_ok()); // Parsed successfully
        let rules: Result<RulesBlock, _> = rules.unwrap().next().unwrap().try_into();
        assert!(rules.is_ok()); // Converted successfully
        assert_eq!(rules.unwrap(),
                   RulesBlock { rules: vec!(
                       TransitionRule {
                           from: "A".into(),
                           to: "B".into(),
                           root: Box::new(
                               RuleASTNode::NotEqual {
                                   lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                   rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                               }
                           )
                       }
                   )}
        );
    }

    #[test]
    fn simple_gt() {
        let rules = LoafParser::parse(Rule::rule_block,
                                      "rule := { from A to B :=  neighborhood(A) > 1 }");
        assert!(rules.is_ok()); // Parsed successfully
        let rules: Result<RulesBlock, _> = rules.unwrap().next().unwrap().try_into();
        assert!(rules.is_ok()); // Converted successfully
        assert_eq!(rules.unwrap(),
            RulesBlock { rules: vec!(
                TransitionRule {
                    from: "A".into(),
                    to: "B".into(),
                    root: Box::new(
                        RuleASTNode::GreaterThan {
                            lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                            rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                        }
                    )
                }
            )}
        );
    }

    #[test]
    fn simple_gte() {
        let rules = LoafParser::parse(Rule::rule_block,
                                      "rule := { from A to B :=  neighborhood(A) >= 1 }");
        assert!(rules.is_ok()); // Parsed successfully
        let rules: Result<RulesBlock, _> = rules.unwrap().next().unwrap().try_into();
        assert!(rules.is_ok()); // Converted successfully
        assert_eq!(rules.unwrap(),
                   RulesBlock { rules: vec!(
                       TransitionRule {
                           from: "A".into(),
                           to: "B".into(),
                           root: Box::new(
                               RuleASTNode::GreaterThanOrEqualTo {
                                   lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                   rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                               }
                           )
                       }
                   )}
        );
    }

    #[test]
    fn simple_lt() {
        let rules = LoafParser::parse(Rule::rule_block,
                                      "rule := { from A to B :=  neighborhood(A) < 1 }");
        assert!(rules.is_ok()); // Parsed successfully
        let rules: Result<RulesBlock, _> = rules.unwrap().next().unwrap().try_into();
        assert!(rules.is_ok()); // Converted successfully
        assert_eq!(rules.unwrap(),
                   RulesBlock { rules: vec!(
                       TransitionRule {
                           from: "A".into(),
                           to: "B".into(),
                           root: Box::new(
                               RuleASTNode::LessThan {
                                   lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                   rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                               }
                           )
                       }
                   )}
        );
    }

    #[test]
    fn simple_lte() {
        let rules = LoafParser::parse(Rule::rule_block,
                                      "rule := { from A to B :=  neighborhood(A) <= 1 }");
        assert!(rules.is_ok()); // Parsed successfully
        let rules: Result<RulesBlock, _> = rules.unwrap().next().unwrap().try_into();
        assert!(rules.is_ok()); // Converted successfully
        assert_eq!(rules.unwrap(),
                   RulesBlock { rules: vec!(
                       TransitionRule {
                           from: "A".into(),
                           to: "B".into(),
                           root: Box::new(
                               RuleASTNode::LessThanOrEqualTo {
                                   lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                   rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                               }
                           )
                       }
                   )}
        );
    }

    #[test]
    fn simple_and() {
        let rules = LoafParser::parse(Rule::rule_block,
                                      "rule := { from A to B :=  neighborhood(A) = 1 and neighborhood(B) = 2 }");
        assert!(rules.is_ok()); // Parsed successfully
        let rules: Result<RulesBlock, _> = rules.unwrap().next().unwrap().try_into();
        assert!(rules.is_ok()); // Converted successfully
        assert_eq!(rules.unwrap(),
                   RulesBlock { rules: vec!(
                       TransitionRule {
                           from: "A".into(),
                           to: "B".into(),
                           root: Box::new(
                               RuleASTNode::And {
                                   lhs: Box::new(RuleASTNode::Equal {
                                       lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                       rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                                   }),
                                   rhs: Box::new(RuleASTNode::Equal {
                                       lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("B".into()))),
                                       rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(2)))
                                   }),
                               }
                           )
                       }
                   )}
        );
    }

    #[test]
    fn simple_or() {
        let rules = LoafParser::parse(Rule::rule_block,
                                      "rule := { from A to B :=  neighborhood(A) = 1 or neighborhood(B) = 2 }");
        assert!(rules.is_ok()); // Parsed successfully
        let rules: Result<RulesBlock, _> = rules.unwrap().next().unwrap().try_into();
        assert!(rules.is_ok()); // Converted successfully
        assert_eq!(rules.unwrap(),
                   RulesBlock { rules: vec!(
                       TransitionRule {
                           from: "A".into(),
                           to: "B".into(),
                           root: Box::new(
                               RuleASTNode::Or {
                                   lhs: Box::new(RuleASTNode::Equal {
                                       lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                                       rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                                   }),
                                   rhs: Box::new(RuleASTNode::Equal {
                                       lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("B".into()))),
                                       rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(2)))
                                   }),
                               }
                           )
                       }
                   )}
        );
    }
}