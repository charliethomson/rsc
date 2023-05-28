use log::trace;
use pest::iterators::Pair;

use crate::{
    parser::{
        ast::Parse,
        error::{ParseError, ParseResult},
    },
    Rule,
};

#[derive(Debug, Clone)]
pub enum Operator {
    // add
    Add,
    // subtract
    Subtract,
    // multiply
    Multiply,
    // divide
    Divide,
    // pow
    Pow,
    // mod
    Mod,
    // and
    And,
    // or
    Or,
    // bit_and
    BitAnd,
    // bit_or
    BitOr,
    // bit_xor
    BitXor,
    // field_access
    FieldAccess,
    // eq
    Eq,
    // neq
    Neq,
    // greater
    Greater,
    // lesser
    Lesser,
    // greater_eq
    GreaterEq,
    // lesser_eq
    LesserEq,
    // inc | post_inc
    Inc,
    // dec | post_dec
    Dec,
    // not
    Not,
    // bit_not
    BitNot,
    // assign
    Assign,
    // comma
    Comma,
}

impl Parse for Operator {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("create-operator({:?})", line.as_rule());
        match line.as_rule() {
            Rule::add => Ok(Self::Add),
            Rule::subtract => Ok(Self::Subtract),
            Rule::multiply => Ok(Self::Multiply),
            Rule::divide => Ok(Self::Divide),
            Rule::pow => Ok(Self::Pow),
            Rule::r#mod => Ok(Self::Mod),
            Rule::and => Ok(Self::And),
            Rule::or => Ok(Self::Or),
            Rule::bit_and => Ok(Self::BitAnd),
            Rule::bit_or => Ok(Self::BitOr),
            Rule::bit_xor => Ok(Self::BitXor),
            Rule::field_access => Ok(Self::FieldAccess),
            Rule::eq => Ok(Self::Eq),
            Rule::neq => Ok(Self::Neq),
            Rule::greater => Ok(Self::Greater),
            Rule::lesser => Ok(Self::Lesser),
            Rule::greater_eq => Ok(Self::GreaterEq),
            Rule::lesser_eq => Ok(Self::LesserEq),
            Rule::inc | Rule::post_inc => Ok(Self::Inc),
            Rule::dec | Rule::post_dec => Ok(Self::Dec),
            Rule::not => Ok(Self::Not),
            Rule::bit_not => Ok(Self::BitNot),
            Rule::assign => Ok(Self::Assign),
            Rule::comma => Ok(Self::Comma),
            rule => {
                return Err(ParseError::InvalidRuleErrorOneOf {
                    expected: vec![
                        Rule::add,
                        Rule::subtract,
                        Rule::multiply,
                        Rule::divide,
                        Rule::pow,
                        Rule::r#mod,
                        Rule::and,
                        Rule::or,
                        Rule::bit_and,
                        Rule::bit_or,
                        Rule::bit_xor,
                        Rule::field_access,
                        Rule::eq,
                        Rule::neq,
                        Rule::greater,
                        Rule::lesser,
                        Rule::greater_eq,
                        Rule::lesser_eq,
                        Rule::inc,
                        Rule::post_inc,
                        Rule::dec,
                        Rule::post_dec,
                        Rule::not,
                        Rule::bit_not,
                        Rule::assign,
                        Rule::comma,
                    ],
                    actual: rule,
                })
            }
        }
    }
}
