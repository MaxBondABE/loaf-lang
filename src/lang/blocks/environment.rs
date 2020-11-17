use crate::lang::parse::{LoafPair, Rule, Error as ParseError};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum EnvironmentBlock {
    Grid1D { x: Option<usize> },
    Grid2D { x: Option<usize>, y: Option<usize>},
    Grid3D { x: Option<usize>, y: Option<usize>, z: Option<usize>}
}
impl TryFrom<LoafPair<'_>> for EnvironmentBlock {
    type Error = ParseError;

    fn try_from(pair: LoafPair) -> Result<Self, Self::Error> {
        debug_assert_eq!(pair.as_rule(), Rule::environment_block);
        let pair = pair.into_inner().next().expect("Environment block has exactly 1 child.");
        match pair.as_rule() {
            Rule::builtin_environments => {
                let mut x: Option<usize> = None;
                let mut y: Option<usize> = None;
                let mut z: Option<usize> = None;

                let mut children = pair.into_inner();
                let env_rule = children.next().expect("Builtin environments has at lead 1 child.");
                let env_dims = children.next();
                if env_dims.is_some() {
                    for dim_directive in env_dims.unwrap().into_inner() {
                        let mut dim_children = dim_directive.into_inner();
                        let dimension = dim_children.next().expect("Dimension directive has exactly 2 children.");
                        let magnitude = Some(
                            usize::from_str(
                                dim_children.next().expect("Dimension directive has exactly 2 children.").as_str()
                            )?
                        );
                        match dimension.as_rule() {
                            Rule::x_dimension => {
                                x = magnitude;
                            },
                            Rule::y_dimension => {
                                y = magnitude;
                            },
                            Rule::z_dimension => {
                                z = magnitude;
                            },
                            Rule::all_dimensions => {
                                x = magnitude;
                                y = magnitude;
                                z = magnitude;
                            },
                            _ => unreachable!()
                        }
                    }
                }
                match env_rule.as_rule() {
                    Rule::env_1D => {
                        Ok(Self::Grid1D {x})
                    },
                    Rule::env_2D => {
                        Ok(Self::Grid2D {x, y})
                    },
                    Rule::env_3D => {
                        Ok(Self::Grid3D {x, y, z})
                    },
                    _ => unreachable!()
                }
            },
            Rule::adjacency_matrix => unimplemented!(),
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lang::parse::LoafParser;
    use pest::Parser;
    use std::convert::TryInto;

    #[test]
    fn grid_1d_no_dim() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 1D");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid1D {x: None});
    }

    #[test]
    fn grid_1d_with_explicit_dim() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 1D::(x = 1)");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid1D {x: Some(1)});
    }


    #[test]
    fn grid_1d_with_all_dim() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 1D::(* = 1)");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid1D {x: Some(1)});
    }


    #[test]
    fn grid_2d_no_dim() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 2D");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid2D {x: None, y: None});
    }

    #[test]
    fn grid_2d_with_explicit_dims() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 2D::(x = 1, y = 2)");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid2D {x: Some(1), y: Some(2)});
    }

    #[test]
    fn grid_2d_with_all_dims() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 2D::(* = 1)");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid2D {x: Some(1), y: Some(1)});
    }

    #[test]
    fn grid_3d_no_dim() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 3D");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid3D {x: None, y: None, z: None});
    }

    #[test]
    fn grid_3d_with_explicit_dims() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 3D::(x = 1, y = 2, z = 3)");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid3D {x: Some(1), y: Some(2), z: Some(3)});
    }


    #[test]
    fn grid_3d_with_all_dims() {
        let env = LoafParser::parse(Rule::environment_block, "environment := 3D::(* = 1)");
        assert!(env.is_ok()); // Parsed successfully
        let env: Result<EnvironmentBlock,_> = env.unwrap().next().unwrap().try_into();
        assert!(env.is_ok()); // Converted successfully
        assert_eq!(env.unwrap(), EnvironmentBlock::Grid3D {x: Some(1), y: Some(1), z: Some(1)});
    }
}