use pest::iterators::Pair;

use crate::Rule;

use super::error::ParseResult;

pub mod context;
pub mod expr;
pub mod function;
pub mod function_parameter;
pub mod ident;
pub mod statement;
pub mod type_definition;

pub trait Parse: Sized {
    fn parse(line: Pair<Rule>) -> ParseResult<Self>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub content: String,
    pub start: usize,
    pub end: usize,
}

pub fn span(line: &Pair<Rule>) -> Span {
    let s = line.as_span();
    Span {
        content: s.as_str().to_owned(),
        end: s.end(),
        start: s.start(),
    }
}
