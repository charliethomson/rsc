use pest::iterators::Pair;

use crate::{
    parser::{
        ast::{ident::Ident, Parse},
        error::{missing, ParseError, ParseResult},
    },
    Rule,
};

use super::literal::Literal;

#[derive(Debug, Clone)]
pub enum Atom {
    Literal(Literal),
    Ident(Ident),
}
impl Parse for Atom {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        match line.as_rule() {
            Rule::literal => {
                let rule = line.into_inner().next().ok_or(missing("atom:literal"))?;
                return Ok(Self::Literal(Literal::parse(rule)?));
            }
            Rule::ident => {
                return Ok(Self::Ident(Ident::parse(line)?));
            }
            rule => {
                return Err(ParseError::InvalidRuleErrorOneOf {
                    expected: vec![Rule::literal, Rule::ident],
                    actual: rule,
                })
            }
        }
    }
}
