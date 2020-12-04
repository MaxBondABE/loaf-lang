use crate::lang::parse::{LoafPair, Rule, Error as ParseError};
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq)]
pub enum BoundaryBlock {
    Void,
    Wrap,
    Infinite,
    Static(Option<String>)
}

impl BoundaryBlock {
    pub fn is_finite(&self) -> bool {
        *self != BoundaryBlock::Infinite
    }
    pub fn is_static(&self) -> Option<&String> {
        match self {
            BoundaryBlock::Static(name) => name.as_ref(),
            _ => None
        }
    }
}

impl TryFrom<LoafPair<'_>> for BoundaryBlock {
    type Error = ParseError;

    fn try_from(pair: LoafPair) -> Result<Self, Self::Error> {
        debug_assert_eq!(pair.as_rule(), Rule::boundary_block);
        let child = pair
            .into_inner()
            .next()
            .expect("Boundary blocks should always have exactly one child.");
        match child.as_rule() {
            Rule::void_boundary => Ok(Self::Void),
            Rule::wrap_boundary => Ok(Self::Wrap),
            Rule::infinite_boundary => Ok(Self::Infinite),
            Rule::static_boundary => {
                let name = child.into_inner().next().map(|p| p.as_str().into());
                Ok(Self::Static(name))
            },
            _ => unreachable!()
        }
    }
}
impl Default for BoundaryBlock {
    fn default() -> Self { Self::Void }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lang::parse::LoafParser;
    use pest::Parser;
    use std::convert::TryInto;

    #[test]
    fn void_boundary() {
        let bb = LoafParser::parse(Rule::boundary_block, "boundary := void");
        assert!(bb.is_ok()); // Parsed successfully
        let bb: Result<BoundaryBlock,_> = bb.unwrap().next().unwrap().try_into();
        assert!(bb.is_ok()); // Converted successfully
        let bb = bb.unwrap();
        assert_eq!(bb, BoundaryBlock::Void);
    }

    #[test]
    fn wrap_boundary() {
        let bb = LoafParser::parse(Rule::boundary_block, "boundary := wrap");
        assert!(bb.is_ok()); // Parsed successfully
        let bb: Result<BoundaryBlock,_> = bb.unwrap().next().unwrap().try_into();
        assert!(bb.is_ok()); // Converted successfully
        let bb = bb.unwrap();
        assert_eq!(bb, BoundaryBlock::Wrap);
    }

    #[test]
    fn infinite_boundary() {
        let bb = LoafParser::parse(Rule::boundary_block, "boundary := infinite");
        assert!(bb.is_ok()); // Parsed successfully
        let bb: Result<BoundaryBlock,_> = bb.unwrap().next().unwrap().try_into();
        assert!(bb.is_ok()); // Converted successfully
        let bb = bb.unwrap();
        assert_eq!(bb, BoundaryBlock::Infinite);
    }

    #[test]
    fn static_default_boundary() {
        let bb = LoafParser::parse(Rule::boundary_block, "boundary := static");
        assert!(bb.is_ok()); // Parsed successfully
        let bb: Result<BoundaryBlock,_> = bb.unwrap().next().unwrap().try_into();
        assert!(bb.is_ok()); // Converted successfully
        let bb = bb.unwrap();
        assert_eq!(bb, BoundaryBlock::Static(None));
    }

    #[test]
    fn static_boundary_with_state() {
        let bb = LoafParser::parse(Rule::boundary_block, "boundary := static::(StateName)");
        assert!(bb.is_ok()); // Parsed successfully
        let bb: Result<BoundaryBlock,_> = bb.unwrap().next().unwrap().try_into();
        assert!(bb.is_ok()); // Converted successfully
        let bb = bb.unwrap();
        assert_eq!(bb, BoundaryBlock::Static(Some("StateName".into())));
    }
}