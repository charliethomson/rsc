use pest::iterators::Pair;
use snailquote::unescape;

use crate::{
    parser::{
        ast::Parse,
        error::{missing, ParseError, ParseResult},
    },
    validate_rule, Rule,
};

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i32),
    Float(f32),
    String(String),
    Char(String),
    Bool(bool),
}
impl Parse for Literal {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        match line.as_rule() {
            Rule::number => {
                let raw = line.as_str().to_owned();
                let maybe_int = raw.parse::<i32>();
                if let Ok(int) = maybe_int {
                    return Ok(Self::Integer(int));
                }

                return Ok(Self::Float(raw.parse::<f32>()?));
            }
            Rule::string => {
                let inner = line
                    .into_inner()
                    .next()
                    .ok_or(missing("literal:string(raw_string)"))?;
                validate_rule!(inner.as_rule(), raw_string);
                let raw = inner.as_str().to_owned();
                return Ok(Self::String(raw));
            }
            Rule::chr => {
                let inner = line
                    .into_inner()
                    .next()
                    .ok_or(missing("literal:chr(raw_chr)"))?;
                validate_rule!(inner.as_rule(), raw_chr);
                let raw = unescape(inner.as_str())?.to_owned();
                return Ok(Self::Char(raw));
            }
            Rule::bool => match line.as_str() {
                "true" => return Ok(Self::Bool(true)),
                "false" => return Ok(Self::Bool(false)),
                _ => unreachable!(),
            },
            rule => {
                return Err(ParseError::InvalidRuleErrorOneOf {
                    expected: vec![Rule::number, Rule::string, Rule::chr, Rule::bool],
                    actual: rule,
                })
            }
        }
    }
}
