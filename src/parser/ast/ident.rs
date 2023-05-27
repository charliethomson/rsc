use log::trace;
use pest::iterators::Pair;

use crate::{
    parser::error::{ParseError, ParseResult},
    Rule,
};

use super::Parse;

#[derive(Debug, Clone)]
pub enum Ident {
    Identifier(String),
    Type(String),
}
impl Parse for Ident {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-ident");

        let rule = line.as_rule();

        trace!("[Start:1] validate-rule");
        if matches!(rule, Rule::typ) {
            trace!("[EndOf:1] validate-rule (typ)");

            trace!("[Start:2] construct-type");
            let ident = line.as_str().to_owned();
            trace!("[EndOf:2] construct-type");

            trace!("[EndOf] parse-ident");
            Ok(Self::Type(ident))
        } else if matches!(rule, Rule::ident) {
            trace!("[EndOf:1] validate-rule (ident)");

            trace!("[Start:2] construct-ident");
            let ident = line.as_str().to_owned();
            trace!("[EndOf:2] construct-ident");

            trace!("[EndOf] parse-ident");
            Ok(Self::Identifier(ident))
        } else {
            trace!(
                "[EndOf] invalid-rule: Expected ident or typ, got {:?}",
                rule
            );
            return ParseResult::Err(ParseError::InvalidRuleError {
                expected: Rule::ident,
                actual: rule,
            });
        }
    }
}
