use std::collections::HashMap;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

use crate::datatypes::neighborhood::Neighborhood;
use crate::datatypes::state::State;

// TODO Debug, Clone

pub struct Ruleset<S: State, N: Neighborhood<S>> {
    rules: HashMap<S, (ASTRoot<S, N>, S)>,
}
impl<S: State, N: Neighborhood<S>> Ruleset<S, N> {
    pub fn new(rules: Vec<(S, (ASTRoot<S, N>, S))>) -> Self {
        Self {
            rules: rules.into_iter().collect(),
        }
    }

    pub fn transition(&self, from_state: S, neighborhood: N) -> Option<S> {
        let (rule, to_state) = &self.rules[&from_state];
        if rule.evaluate(neighborhood) {
            Some(*to_state)
        } else {
            None
        }
    }
}

pub trait ASTNode<S: State, N: Neighborhood<S>> {
    fn evaluate(&self, neighborhood: &N) -> LoafType;
}

pub struct ASTRoot<S: State, N: Neighborhood<S>> {
    child: Box<dyn ASTNode<S, N>>,
    phantom: PhantomData<(S, N)>,
}
impl<S: State, N: Neighborhood<S>> ASTRoot<S, N> {
    pub fn new(child: Box<dyn ASTNode<S, N>>) -> Self {
        Self {
            child,
            phantom: PhantomData,
        }
    }

    pub fn evaluate(&self, neighborhood: N) -> bool {
        self.child.evaluate(&neighborhood).into()
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum LoafType {
    Boolean(bool),
    Integer(isize),
}
impl LoafType {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
impl Into<bool> for LoafType {
    fn into(self) -> bool {
        match self {
            LoafType::Boolean(b) => b,
            _ => panic!("Attempted to cast a nonboolean to bool"),
        }
    }
}
impl Into<isize> for LoafType {
    fn into(self) -> isize {
        match self {
            LoafType::Integer(i) => i,
            _ => panic!("Attempted to cast a noninteger to integer"),
        }
    }
}
impl From<bool> for LoafType {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}
impl From<isize> for LoafType {
    fn from(i: isize) -> Self {
        Self::Integer(i)
    }
}
impl From<usize> for LoafType {
    fn from(i: usize) -> Self {
        Self::Integer(isize::try_from(i).expect("Failed to convert isize to usize"))
    }
}

// TODO error message implies overflow but it could be underflow?
// TODO give integers during error?
impl Add<LoafType> for LoafType {
    type Output = Self;

    fn add(self, rhs: LoafType) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => a
                .checked_add(b)
                .expect("Integer overflow during addition")
                .into(),
            _ => panic!("Attempted to perform addition on noninteger"),
        }
    }
}
impl Sub<LoafType> for LoafType {
    type Output = Self;

    fn sub(self, rhs: LoafType) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => a
                .checked_sub(b)
                .expect("Integer overflow during substraction")
                .into(),
            _ => panic!("Attempted to perform addition on noninteger"),
        }
    }
}
impl Mul<LoafType> for LoafType {
    type Output = Self;

    fn mul(self, rhs: LoafType) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => a
                .checked_mul(b)
                .expect("Integer overflow during multiplication")
                .into(),
            _ => panic!("Attempted to perform addition on noninteger"),
        }
    }
}
impl Div<LoafType> for LoafType {
    type Output = Self;

    fn div(self, rhs: LoafType) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => {
                if b == 0 {
                    panic!("Attempted to divide by 0")
                }
                a.checked_div(b)
                    .expect("Integer overflow during division")
                    .into()
            }
            _ => panic!("Attempted to perform addition on noninteger"),
        }
    }
}
impl<S: State, N: Neighborhood<S>> ASTNode<S, N> for LoafType {
    fn evaluate(&self, _neighborhood: &N) -> LoafType {
        *self
    }
}

macro_rules! binary_operations {
    ( $($name:ident : $logic:expr)* ) => {$(
        pub struct $name<S: State, N: Neighborhood<S>> {
            lhs: Box<dyn ASTNode<S, N>>,
            rhs: Box<dyn ASTNode<S, N>>
        }
        impl<S: State, N: Neighborhood<S>> $name<S, N> {
            pub fn new(lhs: Box<dyn ASTNode<S, N>>, rhs: Box<dyn ASTNode<S, N>>) -> Self {
                Self { lhs, rhs }
            }
            pub fn boxed(self) -> Box<Self> {
                Box::new(self)
            }
        }
        impl<S: State, N: Neighborhood<S>> ASTNode<S, N> for $name<S, N> {
            fn evaluate(&self, neighborhood: &N) -> LoafType {
                let f: fn(LoafType, LoafType) -> LoafType = $logic;
                (f)(self.lhs.evaluate(&neighborhood), self.rhs.evaluate(&neighborhood))
            }
        }
    )*}
}

binary_operations!(
    AddNode: |lhs, rhs| lhs + rhs
    SubNode: |lhs, rhs| lhs - rhs
    MulNode: |lhs, rhs| lhs * rhs
    DivNode: |lhs, rhs| lhs / rhs
    EqNode: |lhs, rhs| (lhs == rhs).into()
    NeqNode: |lhs, rhs| (lhs != rhs).into()
    GtNode: |lhs, rhs| (lhs > rhs).into()
    GteNode: |lhs, rhs| (lhs >= rhs).into()
    LtNode: |lhs, rhs| (lhs < rhs).into()
    LteNode: |lhs, rhs| (lhs <= rhs).into()
    AndNode: |lhs, rhs| (lhs.into() && rhs.into()).into()
    OrNode: |lhs, rhs| (lhs.into() || rhs.into()).into()
);

#[derive(Debug, Clone)]
pub struct CensusNode<S: State> {
    state: S,
}
impl<S: State> CensusNode<S> {
    pub fn new(state: S) -> Self {
        Self { state }
    }
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
impl<S: State, N: Neighborhood<S>> ASTNode<S, N> for CensusNode<S> {
    fn evaluate(&self, neighborhood: &N) -> LoafType {
        neighborhood.count(self.state).into()
    }
}

#[cfg(test)]
pub mod state_rules_tests {
    use super::*;

    #[test]
    fn add_loaf_type() {
        assert_eq!(
            LoafType::Integer(10) + LoafType::Integer(5),
            LoafType::Integer(15)
        )
    }

    #[test]
    fn sub_loaf_type() {
        assert_eq!(
            LoafType::Integer(10) - LoafType::Integer(5),
            LoafType::Integer(5)
        )
    }

    #[test]
    fn mul_loaf_type() {
        assert_eq!(
            LoafType::Integer(10) * LoafType::Integer(5),
            LoafType::Integer(50)
        )
    }

    #[test]
    fn div_loaf_type() {
        assert_eq!(
            LoafType::Integer(10) / LoafType::Integer(5),
            LoafType::Integer(2)
        )
    }

    #[test]
    fn add_rule() {
        assert_eq!(
            AddNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(5).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Integer(15)
        )
    }

    #[test]
    fn sub_rule() {
        assert_eq!(
            SubNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(5).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Integer(5)
        )
    }

    #[test]
    fn mul_rule() {
        assert_eq!(
            MulNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(5).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Integer(50)
        )
    }

    #[test]
    fn div_rule() {
        assert_eq!(
            DivNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(5).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Integer(2)
        )
    }

    #[test]
    fn gt_rule_true_when_lhs_gt_rhs() {
        assert_eq!(
            GtNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(5).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn gt_rule_false_when_lhs_lt_rhs() {
        assert_eq!(
            GtNode::new(LoafType::Integer(5).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }
    #[test]
    fn gt_rule_false_when_lhs_eq_rhs() {
        assert_eq!(
            GtNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }

    #[test]
    fn gte_rule_true_when_lhs_gt_rhs() {
        assert_eq!(
            GteNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(5).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn gte_rule_false_when_lhs_lt_rhs() {
        assert_eq!(
            GteNode::new(LoafType::Integer(5).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }
    #[test]
    fn gte_rule_true_when_lhs_eq_rhs() {
        assert_eq!(
            GteNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }

    #[test]
    fn lt_rule_false_when_lhs_gt_rhs() {
        assert_eq!(
            LtNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(5).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }
    #[test]
    fn lt_rule_false_when_lhs_lt_rhs() {
        assert_eq!(
            LtNode::new(LoafType::Integer(5).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn lt_rule_false_when_lhs_eq_rhs() {
        assert_eq!(
            LtNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }

    #[test]
    fn lte_rule_false_when_lhs_gt_rhs() {
        assert_eq!(
            LteNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(5).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }
    #[test]
    fn lte_rule_false_when_lhs_lt_rhs() {
        assert_eq!(
            LteNode::new(LoafType::Integer(5).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn lte_rule_true_when_lhs_eq_rhs() {
        assert_eq!(
            LteNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }

    #[test]
    fn eq_rule_true_when_lhs_eq_rhs() {
        assert_eq!(
            EqNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn eq_rule_false_when_lhs_neq_rhs() {
        assert_eq!(
            EqNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(11).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }

    #[test]
    fn neq_rule_true_when_lhs_eq_rhs() {
        assert_eq!(
            NeqNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(10).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }
    #[test]
    fn neq_rule_true_when_lhs_neq_rhs() {
        assert_eq!(
            NeqNode::new(LoafType::Integer(10).boxed(), LoafType::Integer(11).boxed())
                .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }

    #[test]
    fn and_rule_true_for_t_t() {
        assert_eq!(
            AndNode::new(
                LoafType::Boolean(true).boxed(),
                LoafType::Boolean(true).boxed()
            )
            .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn and_rule_false_for_t_f() {
        assert_eq!(
            AndNode::new(
                LoafType::Boolean(true).boxed(),
                LoafType::Boolean(false).boxed()
            )
            .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }
    #[test]
    fn and_rule_false_for_f_t() {
        assert_eq!(
            AndNode::new(
                LoafType::Boolean(false).boxed(),
                LoafType::Boolean(true).boxed()
            )
            .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }
    #[test]
    fn and_rule_false_for_f_f() {
        assert_eq!(
            AndNode::new(
                LoafType::Boolean(false).boxed(),
                LoafType::Boolean(false).boxed()
            )
            .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }

    #[test]
    fn or_rule_true_for_t_t() {
        assert_eq!(
            OrNode::new(
                LoafType::Boolean(true).boxed(),
                LoafType::Boolean(true).boxed()
            )
            .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn or_rule_true_for_t_f() {
        assert_eq!(
            OrNode::new(
                LoafType::Boolean(true).boxed(),
                LoafType::Boolean(false).boxed()
            )
            .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn or_rule_false_for_f_t() {
        assert_eq!(
            OrNode::new(
                LoafType::Boolean(false).boxed(),
                LoafType::Boolean(true).boxed()
            )
            .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(true)
        );
    }
    #[test]
    fn or_rule_false_for_f_f() {
        assert_eq!(
            OrNode::new(
                LoafType::Boolean(false).boxed(),
                LoafType::Boolean(false).boxed()
            )
            .evaluate(&Vec::<usize>::new()),
            LoafType::Boolean(false)
        );
    }

    #[test]
    fn census_op() {
        assert_eq!(
            CensusNode::new(0usize).evaluate(&vec!(0, 0, 0, 1, 1, 2)),
            LoafType::Integer(3)
        );
        assert_eq!(
            CensusNode::new(1usize).evaluate(&vec!(0, 0, 0, 1, 1, 2)),
            LoafType::Integer(2)
        );
        assert_eq!(
            CensusNode::new(2usize).evaluate(&vec!(0, 0, 0, 1, 1, 2)),
            LoafType::Integer(1)
        );
    }

    #[test]
    fn test_realistic_ast() {
        assert_eq!(
            GtNode::new(
                AddNode::new(
                    CensusNode::new(0).boxed().boxed(),
                    LoafType::Integer(1).boxed()
                )
                .boxed(),
                CensusNode::new(1).boxed()
            )
            .evaluate(&vec!(0usize, 0usize, 1usize, 1usize)),
            LoafType::Boolean(true)
        );
        assert_eq!(
            GtNode::new(
                AddNode::new(
                    CensusNode::new(0).boxed().boxed(),
                    LoafType::Integer(1).boxed()
                )
                .boxed(),
                CensusNode::new(1).boxed()
            )
            .evaluate(&vec!(0usize, 0usize, 1usize, 1usize, 1usize)),
            LoafType::Boolean(false)
        );
    }
}
