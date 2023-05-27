use pest::iterators::Pair;

use crate::Rule;

use super::error::ParseResult;

pub mod expr;
pub mod function;
pub mod function_parameter;
pub mod ident;
pub mod statement;

pub trait Parse: Sized {
    fn parse(line: Pair<Rule>) -> ParseResult<Self>;
}
