use log::trace;
use pest::iterators::Pair;

use crate::{next, parser::error::ParseResult, validate_rule, Rule};

use super::{function_parameter::FunctionParameter, ident::Ident, statement::Statement, Parse};

#[derive(Debug)]
pub struct Function {
    pub func_name: Ident,
    pub params: Vec<FunctionParameter>,
    pub return_type: Option<Ident>,
    pub body: Vec<Statement>,
}
impl Parse for Function {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-function");

        let rule = line.as_rule();
        trace!("[Start:1] validate-rule");
        validate_rule!(rule, function);
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] get-rules");
        let mut rules = line.into_inner();
        let ident = next!(rules, "function(ident)");
        let params = next!(rules, "function(params)");
        let maybe_return_type = next!(rules, "function(return-type-or-body)");
        let (return_type, body) = if matches!(maybe_return_type.as_rule(), Rule::ident) {
            (Some(maybe_return_type), next!(rules, "function(body)"))
        } else {
            (None, maybe_return_type)
        };
        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] parse-params");
        let params = params.into_inner();
        let params = params
            .map(FunctionParameter::parse)
            .collect::<ParseResult<Vec<_>>>()?;
        trace!("[EndOf:3] parse-params");

        trace!("[Start:4] parse-body");
        validate_rule!(body.as_rule(), stmts);
        let body = body.into_inner();
        let body = body
            .map(Statement::parse)
            .collect::<ParseResult<Vec<_>>>()?;
        trace!("[EndOf:4] parse-body");

        trace!("[Start:5] construct-function");
        let function = Self {
            func_name: Ident::parse(ident)?,
            params,
            return_type: return_type.map(Ident::parse).transpose()?,
            body,
        };
        trace!("[EndOf:5] construct-function");

        trace!("[EndOf] parse-function");
        Ok(function)
    }
}
