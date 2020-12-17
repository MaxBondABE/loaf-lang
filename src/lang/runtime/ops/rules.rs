use crate::lang::parse::blocks::rule::{RulesBlock, RuleASTNode, RuleTerminal};
use crate::lang::runtime::ops::rules::RuleValue::{Boolean, Number};
use crate::lang::runtime::ops::{ToState, FromState};
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use std::collections::HashMap;
use crate::lang::runtime::{StateMap, StateId};

pub struct Rules {
    rules: HashMap<FromState, Vec<(ToState, Box<dyn RuleOperation>)>>
}
impl Rules {
    pub fn from_block(block: RulesBlock, state_map: &StateMap) -> Self {
        let mut rules = HashMap::new();
        for rule in block.into_vec() {
            let from = *state_map.get(&rule.from).expect("State map should be complete.");
            let to= *state_map.get(&rule.to).expect("State map should be complete.");
            let r = build_ast(rule.root, state_map);
            rules.entry(from).or_insert_with(|| Vec::new()).push((to, r));
            // TODO interpret collisions on to as implicit OrOp
        }
        Self { rules }
    }
    pub fn evaluate(&self, state: StateId, neighborhood: Vec<StateId>) -> Option<StateId> {
        for (to_state, rule) in self.rules.get(&state)? {
            if rule.evaluate(&neighborhood).into() {
                return Some(*to_state);
            }
        }
        None
    }
}

// TODO move off of recursive implementation
fn build_ast(node: Box<RuleASTNode>, state_map: &StateMap) -> Box<dyn RuleOperation> {
    match *node {
        RuleASTNode::Terminal(t) => {
            match t {
                RuleTerminal::Number(n) => Box::new(RuleValue::Number(n)),
                RuleTerminal::Census(name) => Census::boxed(
                    *state_map.get(&name).expect("State map should be complete.")
                )
            }
        }
        RuleASTNode::Add { lhs, rhs } => {
            AddOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::Sub { lhs, rhs } => {
            SubOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::Mul { lhs, rhs } => {
            MulOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::Div { lhs, rhs } => {
            DivOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::And { lhs, rhs } => {
            AndOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::Or { lhs, rhs } => {
            OrOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::GreaterThan { lhs, rhs } => {
            GtOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::GreaterThanOrEqualTo { lhs, rhs } => {
            GteOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::LessThan { lhs, rhs } => {
            LtOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::LessThanOrEqualTo { lhs, rhs } => {
            LteOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::Equal { lhs, rhs } => {
            EqOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
        RuleASTNode::NotEqual { lhs, rhs } => {
            NeqOp::boxed(build_ast(lhs, state_map), build_ast(rhs, state_map))
        }
    }
}

pub trait RuleOperation {
    fn evaluate(&self, neighborhood: &Vec<StateId>) -> RuleValue;
}

// TODO Panic on nonsensical comparisons?
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum RuleValue {
    Number(isize),
    Boolean(bool)
}
impl RuleOperation for RuleValue {
    fn evaluate(&self, _: &Vec<StateId>) -> RuleValue {
        *self
    }
}
impl From<bool> for RuleValue {
    fn from(v: bool) -> Self {
        Boolean(v)
    }
}
impl From<isize> for RuleValue {
    fn from(v: isize) -> Self {
        Number(v)
    }
}
impl From<RuleValue> for bool {
    fn from(value: RuleValue) -> Self {
        match value {
            RuleValue::Boolean(v) => v,
            RuleValue::Number(v) => v != 0
        }
    }
}
impl Add for RuleValue {
    type Output = RuleValue;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number(lhs), Number(rhs)) => Number(lhs + rhs),
            _ => panic!("Illegal addition operation: attempted to add non-number(s).")
        }
    }
}
impl Sub for RuleValue {
    type Output = RuleValue;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number(lhs), Number(rhs)) => Number(lhs - rhs),
            _ => panic!("Illegal subtraction operation: attempted to subtract non-number(s).")
        }
    }
}
impl Mul for RuleValue {
    type Output = RuleValue;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number(lhs), Number(rhs)) => Number(lhs * rhs),
            _ => panic!("Illegal multiplication operation: attempted to multiply non-number(s).")
        }
    }
}
impl Div for RuleValue {
    type Output = RuleValue;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number(lhs), Number(rhs)) => Number(lhs / rhs),
            _ => panic!("Illegal division operation: attempted to divide non-number(s).")
        }
    }
}

macro_rules! binary_operations {
    ( $($name:ident : $logic:expr)* ) => {$(
        pub struct $name {
            lhs: Box<dyn RuleOperation>,
            rhs: Box<dyn RuleOperation>
        }
        impl $name {
            pub fn new(lhs: Box<dyn RuleOperation>, rhs: Box<dyn RuleOperation>) -> Self {
                Self { lhs, rhs }
            }
            pub fn boxed(lhs: Box<dyn RuleOperation>, rhs: Box<dyn RuleOperation>) -> Box<Self> {
                Box::new(Self::new(lhs, rhs))
            }
        }
        impl RuleOperation for $name {
            fn evaluate(&self, neighborhood: &Vec<StateId>) -> RuleValue {
                let f: fn(RuleValue, RuleValue) -> RuleValue = $logic;
                (f)(self.lhs.evaluate(neighborhood), self.rhs.evaluate(neighborhood))
            }
        }
    )*}
}

// Suffixed to avoid collisions with std::ops traits
binary_operations!(
    AddOp: |lhs, rhs| lhs + rhs
    SubOp: |lhs, rhs| lhs - rhs
    MulOp: |lhs, rhs| lhs * rhs
    DivOp: |lhs, rhs| lhs / rhs
    EqOp: |lhs, rhs| (lhs == rhs).into()
    NeqOp: |lhs, rhs| (lhs != rhs).into()
    GtOp: |lhs, rhs| (lhs > rhs).into()
    GteOp: |lhs, rhs| (lhs >= rhs).into()
    LtOp: |lhs, rhs| (lhs < rhs).into()
    LteOp: |lhs, rhs| (lhs <= rhs).into()
    AndOp: |lhs, rhs| (lhs.into() && rhs.into()).into()
    OrOp: |lhs, rhs| (lhs.into() || rhs.into()).into()
);

pub struct Census {
    state_id: StateId
}
impl Census {
    pub fn new(state_id: StateId) -> Self {
        Self { state_id }
    }
    pub fn boxed(state_id: StateId) -> Box<Self> {
        Box::new(Self::new(state_id))
    }
}
impl RuleOperation for Census {
    fn evaluate(&self, neighborhood: &Vec<StateId>) -> RuleValue {
        RuleValue::Number(
            neighborhood.iter().filter(|s| **s == self.state_id).count() as isize
        )
    }
}

// TODO test combinations of operations
#[cfg(test)]
mod test {
    use super::*;
    use crate::lang::parse::blocks::rule::TransitionRule;
    use crate::lang::parse::blocks::rule::RuleASTNode::Terminal;
    #[macro_use]
    use lazy_static::lazy_static;

    const STATE_A:StateId = 0;
    const STATE_B:StateId = 1;

    lazy_static!(
        static ref STATE_MAP: StateMap = {
            let mut h = HashMap::new();
            h.insert("A".into(), STATE_A);
            h.insert("B".into(), STATE_B);
            h
        };
    );

    // Operations

    #[test]
    fn add_constants() {
        let ops = AddOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(1))
        );
        assert_eq!(ops.evaluate(&vec!()), RuleValue::Number(2));
    }

    #[test]
    fn sub_constants() {
        let ops = SubOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(1))
        );
        assert_eq!(ops.evaluate(&vec!()), RuleValue::Number(0));
    }

    #[test]
    fn mul_constants() {
        let ops = MulOp::boxed(
            Box::new(RuleValue::Number(5)),
            Box::new(RuleValue::Number(2))
        );
        assert_eq!(ops.evaluate(&vec!()), RuleValue::Number(10));
    }

    #[test]
    fn div_constants() {
        let ops = DivOp::boxed(
            Box::new(RuleValue::Number(10)),
            Box::new(RuleValue::Number(2))
        );
        assert_eq!(ops.evaluate(&vec!()), RuleValue::Number(5));
    }

    #[test]
    fn gt_constants() {
        let ops_true = GtOp::boxed(
            Box::new(RuleValue::Number(2)),
            Box::new(RuleValue::Number(1))
        );
        let ops_false = GtOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(2))
        );
        let ops_false_eq = GtOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(1))
        );
        assert_eq!(ops_true.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_false.evaluate(&vec!()), RuleValue::Boolean(false));
        assert_eq!(ops_false_eq.evaluate(&vec!()), RuleValue::Boolean(false));
    }

    #[test]
    fn gte_constants() {
        let ops_true = GteOp::boxed(
            Box::new(RuleValue::Number(2)),
            Box::new(RuleValue::Number(1))
        );
        let ops_true_eq = GteOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(1))
        );
        let ops_false = GteOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(2))
        );
        assert_eq!(ops_true.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_true_eq.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_false.evaluate(&vec!()), RuleValue::Boolean(false));
    }

    #[test]
    fn lt_constants() {
        let ops_true = LtOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(2))
        );
        let ops_false = LtOp::boxed(
            Box::new(RuleValue::Number(2)),
            Box::new(RuleValue::Number(1))
        );
        let ops_false_eq = LtOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(1))
        );
        assert_eq!(ops_true.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_false.evaluate(&vec!()), RuleValue::Boolean(false));
        assert_eq!(ops_false_eq.evaluate(&vec!()), RuleValue::Boolean(false));
    }

    #[test]
    fn lte_constants() {
        let ops_true = LteOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(2))
        );
        let ops_true_eq = LteOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(1))
        );
        let ops_false = LteOp::boxed(
            Box::new(RuleValue::Number(2)),
            Box::new(RuleValue::Number(1))
        );
        assert_eq!(ops_true.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_true_eq.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_false.evaluate(&vec!()), RuleValue::Boolean(false));
    }

    #[test]
    fn eq_constants() {
        let ops_true = EqOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(1))
        );
        let ops_false = EqOp::boxed(
            Box::new(RuleValue::Number(2)),
            Box::new(RuleValue::Number(1))
        );
        assert_eq!(ops_true.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_false.evaluate(&vec!()), RuleValue::Boolean(false));
    }

    #[test]
    fn neq_constants() {
        let ops_true = NeqOp::boxed(
            Box::new(RuleValue::Number(2)),
            Box::new(RuleValue::Number(1))
        );
        let ops_false = NeqOp::boxed(
            Box::new(RuleValue::Number(1)),
            Box::new(RuleValue::Number(1))
        );
        assert_eq!(ops_true.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_false.evaluate(&vec!()), RuleValue::Boolean(false));
    }

    #[test]
    fn and_constants() {
        let ops_true = AndOp::boxed(
            Box::new(RuleValue::Boolean(true)),
            Box::new(RuleValue::Boolean(true))
        );
        let ops_false = AndOp::boxed(
            Box::new(RuleValue::Boolean(false)),
            Box::new(RuleValue::Boolean(true))
        );
        assert_eq!(ops_true.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_false.evaluate(&vec!()), RuleValue::Boolean(false));
    }

    #[test]
    fn or_constants() {
        let ops_true_both = OrOp::boxed(
            Box::new(RuleValue::Boolean(true)),
            Box::new(RuleValue::Boolean(true))
        );
        let ops_true_one = OrOp::boxed(
            Box::new(RuleValue::Boolean(false)),
            Box::new(RuleValue::Boolean(true))
        );
        let ops_false = OrOp::boxed(
            Box::new(RuleValue::Boolean(false)),
            Box::new(RuleValue::Boolean(false))
        );
        assert_eq!(ops_true_both.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_true_one.evaluate(&vec!()), RuleValue::Boolean(true));
        assert_eq!(ops_false.evaluate(&vec!()), RuleValue::Boolean(false));
    }

    #[test]
    fn census() {
        let ops = Census::boxed(1);
        assert_eq!(ops.evaluate(&vec!(0, 1, 1, 2, 2, 2)), RuleValue::Number(2));
    }

    #[test]
    fn constant() {
        assert_eq!(RuleValue::Number(10).evaluate(&vec!()), RuleValue::Number(10));
    }

    // Rule sets

    #[test]
    fn simple_rules_with_transition() {
        let rules = Rules::from_block(
            RulesBlock::new(vec!(
                TransitionRule {
                    from: "A".into(),
                    to: "B".into(),
                    root: Box::new(RuleASTNode::GreaterThan {
                        lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(2))),
                        rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1)))
                    })
                }
            )),
            &STATE_MAP
        );
        assert_eq!(rules.evaluate(STATE_A, vec!()), Some(STATE_B));
    }

    #[test]
    fn simple_rules_without_transition() {
        let rules = Rules::from_block(
            RulesBlock::new(vec!(
                TransitionRule {
                    from: "A".into(),
                    to: "B".into(),
                    root: Box::new(RuleASTNode::GreaterThan {
                        lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(1))),
                        rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Number(2)))
                    })
                }
            )),
            &STATE_MAP
        );
        assert_eq!(rules.evaluate(STATE_A, vec!()), None);
    }


    #[test]
    fn census_rule_with_transition() {
        let rules = Rules::from_block(
            RulesBlock::new(vec!(
                TransitionRule {
                    from: "A".into(),
                    to: "B".into(),
                    root: Box::new(RuleASTNode::GreaterThan {
                        lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                        rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("B".into())))
                    })
                }
            )),
            &STATE_MAP
        );
        assert_eq!(rules.evaluate(STATE_A, vec!(STATE_A, STATE_A, STATE_B)), Some(STATE_B));
    }

    #[test]
    fn census_rule_without_transition() {
        let rules = Rules::from_block(
            RulesBlock::new(vec!(
                TransitionRule {
                    from: "A".into(),
                    to: "B".into(),
                    root: Box::new(RuleASTNode::GreaterThan {
                        lhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("A".into()))),
                        rhs: Box::new(RuleASTNode::Terminal(RuleTerminal::Census("B".into())))
                    })
                }
            )),
            &STATE_MAP
        );
        assert_eq!(rules.evaluate(STATE_A, vec!(STATE_A, STATE_A, STATE_B, STATE_B)), None);
    }

}
