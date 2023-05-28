use log::trace;
use pest::iterators::Pair;

use crate::{next, parser::error::ParseResult, validate_rule, Rule};

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
        let p1 = next!(rules, "function-parameter(arg1)");
        let p1 = Ident::parse(p1)?;

        let (name, ty) = if p1.is_type() {
            // type
            (None, Some(p1))
        } else {
            if let Some(p2) = rules.next() {
                // name type
                let p2 = Ident::parse_expect_type(p2)?;
                (Some(p1), Some(p2))
            } else {
                // name
                (Some(p1), None)
            }
        };

        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] construct-parameter");
        let parameter = match (name, ty) {
            (Some(name), Some(ty)) => Self::NamedAndTyped { name, ty },
            (Some(name), None) => Self::NamedDynamic { name },
            (None, Some(ty)) => Self::Anonymous { ty },
            _ => unreachable!(),
        };
        trace!("[EndOf:3] construct-parameter");

        trace!("[EndOf] parse-parameter");
        Ok(parameter)
    }
}
