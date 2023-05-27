use log::trace;
use pest::iterators::Pair;

use crate::{
    parser::error::{missing, ParseResult},
    validate_rule, Rule,
};

use super::{ident::Ident, Parse};

#[derive(Debug)]
pub struct FunctionParameter {
    pub name: Ident,
    pub ty: Ident,
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
        let name = rules.next().ok_or(missing("function_parameter(name)"))?;
        let ty = rules.next().ok_or(missing("function_parameter(ty)"))?;
        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] construct-parameter");
        let parameter = Self {
            name: Ident::parse(name)?,
            ty: Ident::parse(ty)?,
        };
        trace!("[EndOf:3] construct-parameter");

        trace!("[EndOf] parse-parameter");
        Ok(parameter)
    }
}
