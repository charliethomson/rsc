use itertools::Itertools;
use log::trace;
use pest::iterators::Pair;

use crate::{
    next,
    parser::error::{missing, ParseResult},
    validate_rule, Rule,
};

use self::{atom::Atom, operator::Operator, pratt::PRATT_PARSER};

use super::{ident::Ident, Parse};

pub mod atom;
pub mod literal;
pub mod operator;
pub mod pratt;

type Primary = ParseResult<Expression>;
type SubExp = Box<Expression>;

#[derive(Debug, Clone)]
pub struct DoBranch {
    pub condition: SubExp,
    pub behavior: SubExp,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Atom(Atom),
    BinaryOperation {
        lhs: SubExp,
        operator: Operator,
        rhs: SubExp,
    },
    PrefixOperation {
        operator: Operator,
        rhs: SubExp,
    },
    PostfixOperation {
        lhs: SubExp,
        operator: Operator,
    },
    Call {
        lhs: SubExp,
        params: Option<SubExp>,
    },
    Assignment {
        name: Ident,
        typ: Option<Ident>,
        value: Option<SubExp>,
    },
    Do {
        branches: Vec<DoBranch>,
        default_branch: DoBranch,
    },
}
impl Expression {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
    pub fn parse_boxed(line: Pair<Rule>) -> ParseResult<Box<Self>> {
        Ok(Box::new(Self::parse(line)?))
    }
}
impl Expression {
    fn parse_do(rule: Pair<Rule>) -> Primary {
        trace!("[Start] expr:parse-do");
        validate_rule!(rule.as_rule(), do_expr, field_definition);

        let rules = rule.into_inner();

        let branches = rules
            .chunks(2)
            .into_iter()
            .map(|mut branch| {
                let condition = next!(branch, "expr-do(branch-condition)");
                let behavior = next!(branch, "expr-do(branch-behavior)");

                Ok(DoBranch {
                    condition: Self::parse_boxed(condition)?,
                    behavior: Self::parse_boxed(behavior)?,
                })
            })
            .collect::<ParseResult<Vec<_>>>()?;

        let is_default_branch = |branch: &&DoBranch| {
            let Self::Atom(ref atom) = *branch.condition else { return false };

            let Atom::Ident(ident) = atom else { return false };

            let Ident::Identifier { name, .. } = ident else { return false };

            name == "_"
        };

        let default_branch = branches
            .iter()
            .find(is_default_branch)
            .ok_or(missing("expr-do(no-default)"))?
            .clone();
        let branches = branches
            .into_iter()
            .filter(|branch| !is_default_branch(&branch))
            .collect::<Vec<_>>();

        trace!("[EndOf] expr:parse-do");
        Ok(Self::Do {
            branches,
            default_branch,
        })
    }

    fn parse_assignment(rule: Pair<Rule>) -> Primary {
        trace!("[Start] expr:parse-assignment");
        validate_rule!(rule.as_rule(), assignment, field_definition);

        let mut rules = rule.into_inner();

        let ident = next!(rules, "expr-assignment(ident)");
        let type_or_value = next!(rules, "expr-assignment(type_or_value)");
        let (typ, value) = if matches!(type_or_value.as_rule(), Rule::expr) {
            (None, Some(type_or_value))
        } else {
            (Some(type_or_value), rules.next())
        };

        let assignment = Self::Assignment {
            name: Ident::parse(ident)?,
            typ: typ.map(|t| Ident::parse_expect_type(t)).transpose()?,
            value: value.map(|v| Self::parse_boxed(v)).transpose()?,
        };

        trace!("[EndOf] expr:parse-assignment");
        Ok(assignment)
    }

    fn map_primary(primary: Pair<Rule>) -> Primary {
        let rule = primary.as_rule();
        trace!("[Start] map-primary({:?})", rule);
        let primary = match primary.as_rule() {
            Rule::expr | Rule::infix_expr => Expression::parse(primary)?,
            Rule::parenthesized_expr => Self::parse(next!(
                primary.into_inner(),
                "map-primary(parenthesized-child)"
            ))?,
            Rule::assignment => Self::parse_assignment(primary)?,
            Rule::do_expr => Self::parse_do(primary)?,
            _ => Self::Atom(Atom::parse(primary)?),
        };

        trace!("[EndOf] map-primary({:?})", rule);
        Ok(primary)
    }

    fn map_infix(lhs: Primary, op: Pair<Rule>, rhs: Primary) -> Primary {
        trace!("[Start] map-infix");
        let operator = Operator::parse(op)?;
        let primary = Self::BinaryOperation {
            lhs: Box::new(lhs?),
            operator,
            rhs: Box::new(rhs?),
        };
        trace!("[EndOf] map-infix");
        Ok(primary)
    }
    fn map_postfix(lhs: Primary, op: Pair<Rule>) -> Primary {
        trace!("[Start] map-postfix");
        match op.as_rule() {
            Rule::call_params => {
                return Ok(Self::Call {
                    lhs: lhs?.boxed(),
                    params: op
                        .into_inner()
                        .next()
                        .map(Self::map_primary)
                        .transpose()?
                        .map(Self::boxed),
                });
            }
            _ => {}
        }

        let operator = Operator::parse(op)?;
        let primary = Self::PostfixOperation {
            operator,
            lhs: Box::new(lhs?),
        };
        trace!("[EndOf] map-postfix");
        Ok(primary)
    }
    fn map_prefix(op: Pair<Rule>, rhs: Primary) -> Primary {
        trace!("[Start] map-prefix");
        let operator = Operator::parse(op)?;
        let primary = Self::PrefixOperation {
            rhs: Box::new(rhs?),
            operator,
        };
        trace!("[EndOf] map-prefix");
        Ok(primary)
    }
}
impl Parse for Expression {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-expr({:?})", line.as_rule());
        let rule = line.as_rule();
        validate_rule!(
            rule,
            expr,
            ident,
            literal,
            field_definition,
            infix_expr,
            ID_anon
        );

        match rule {
            Rule::literal | Rule::ident | Rule::ID_anon => Ok(Self::Atom(Atom::parse(line)?)),
            Rule::parenthesized_expr => {
                Self::parse(next!(line.into_inner(), "map-primary(parenthesized-child)"))
            }
            _ => PRATT_PARSER
                .map_primary(Self::map_primary)
                .map_infix(Self::map_infix)
                .map_postfix(Self::map_postfix)
                .map_prefix(Self::map_prefix)
                .parse(line.into_inner()),
        }
    }
}
