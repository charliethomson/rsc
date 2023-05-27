use log::trace;
use pest::iterators::Pair;

use crate::{
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
        params: SubExp,
    },
    Assignment {
        name: Ident,
        typ: Option<Ident>,
        value: Option<SubExp>,
    },
}
impl Expression {
    pub fn parse_boxed(line: Pair<Rule>) -> ParseResult<Box<Self>> {
        Ok(Box::new(Self::parse(line)?))
    }
}
impl Expression {
    fn parse_call(rule: Pair<Rule>) -> Primary {
        trace!("[Start] expr:parse-call");
        validate_rule!(rule.as_rule(), call);
        let mut rules = rule.into_inner();
        let lhs = rules.next().ok_or(missing("expr-call(ident)"))?;
        let params = rules.next().ok_or(missing("expr-call(params)"))?;

        let call = Self::Call {
            lhs: Expression::parse_boxed(lhs)?,
            params: Expression::parse_boxed(params)?,
        };
        trace!("[EndOf] expr:parse-call");
        Ok(call)
    }
    fn parse_assignment(rule: Pair<Rule>) -> Primary {
        trace!("[Start] expr:parse-assignment");
        validate_rule!(rule.as_rule(), assignment);
        let mut rules = rule.into_inner();

        let ident = rules.next().ok_or(missing("expr-assignment(ident)"))?;
        let type_or_value = rules
            .next()
            .ok_or(missing("expr-assignment(type_or_value)"))?;
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
            Rule::expr => Expression::parse(primary)?,
            Rule::call => Self::parse_call(primary)?,
            Rule::assignment => Self::parse_assignment(primary)?,
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
        let rule = line.as_rule();
        validate_rule!(rule, expr, ident, literal);

        match rule {
            Rule::literal | Rule::ident => Ok(Self::Atom(Atom::parse(line)?)),
            _ => PRATT_PARSER
                .map_primary(Self::map_primary)
                .map_infix(Self::map_infix)
                .map_postfix(Self::map_postfix)
                .map_prefix(Self::map_prefix)
                .parse(line.into_inner()),
        }
    }
}
