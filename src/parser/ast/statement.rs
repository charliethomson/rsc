use log::trace;
use pest::iterators::Pair;

use crate::{
    parser::error::{missing, ParseResult},
    validate_rule, Rule,
};

use super::{expr::Expression, ident::Ident, Parse};

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Box<Expression>),
    // TODO: Maybe merge assn & decl
    Assignment {
        ident: Ident,
        typ: Option<Ident>,
        value: Box<Expression>,
    },
    Declaration {
        ident: Ident,
        typ: Ident,
    },
}
impl Parse for Statement {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-statement");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), stmt, expr);
        trace!("[EndOf:1] validate-rule");

        let stmt = match line.as_rule() {
            Rule::expr => Self::Expression(Expression::parse_boxed(line)?),
            Rule::stmt => {
                let rule = line.into_inner().next().ok_or(missing("stmt(root)"))?;
                match rule.as_rule() {
                    Rule::expr => Self::Expression(Expression::parse_boxed(rule)?),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };

        trace!("[EndOf] parse-statement");
        Ok(stmt)
    }
}
