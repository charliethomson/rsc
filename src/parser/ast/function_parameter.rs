use log::trace;
use pest::iterators::Pair;

use crate::{
    parser::error::{missing, ParseResult},
    validate_rule, Rule,
};

use super::{ident::Ident, Parse};

#[derive(Debug)]
pub enum FunctionParameter {
    NamedAndTyped { name: Ident, ty: Ident },
    NamedDynamic { name: Ident },
    Anonymous { ty: Ident },
}

impl Parse for FunctionParameter {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-parameter");

        let rule = line.as_rule();

        trace!("[Start:1] validate-rule");
        validate_rule!(rule, function_parameter);
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] get-rules");
        let mut rules = line.into_inner();
        let p1 = rules.next();
        let (name, ty) = match p1.as_ref().map(|p| p.as_rule()) {
            None => (None, None),
            Some(Rule::ident) => match rules.next() {
                Some(p2) => {
                    validate_rule!(p2.as_rule(), ident);
                    (p1, Some(p2))
                }
                None => (p1, None),
            },
            Some(Rule::native) => (None, p1),
            _ => unreachable!(),
        };

        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] construct-parameter");
        let parameter = match (name, ty) {
            (Some(name), Some(ty)) => Self::NamedAndTyped {
                name: Ident::parse(name)?,
                ty: Ident::parse(ty)?,
            },
            (Some(name), None) => Self::NamedDynamic {
                name: Ident::parse(name)?,
            },
            (None, Some(ty)) => Self::Anonymous {
                ty: Ident::parse(ty)?,
            },
            _ => unreachable!(),
        };
        trace!("[EndOf:3] construct-parameter");

        trace!("[EndOf] parse-parameter");
        Ok(parameter)
    }
}
