use log::trace;
use pest::iterators::Pair;

use crate::{next, parser::error::ParseResult, validate_rule, Rule};

use super::{expr::Expression, function::Function, ident::Ident, Parse};

#[derive(Debug)]
pub struct TypeDefinition {
    pub name: Ident,
    pub fields: Vec<Expression>,
    pub methods: Vec<Function>,
}
impl TypeDefinition {
    fn parse_field_definition(line: Pair<Rule>) -> ParseResult<Expression> {
        trace!("[Start] parse-field-definition");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), field_definition);
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] get-rules");

        let mut rules = line.into_inner();
        let name = next!(rules, "field-definition(ident)");
        let type_or_value = next!(rules, "field-definition(type_or_value)");
        let (typ, value) = if matches!(type_or_value.as_rule(), Rule::expr) {
            (None, Some(type_or_value))
        } else {
            (Some(type_or_value), rules.next())
        };
        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] parse-name");
        let name = Ident::parse(name)?;
        trace!("[EndOf:3] parse-name");

        trace!("[Start:4] parse-type");
        let typ = typ.map(Ident::parse_expect_type).transpose()?;
        trace!("[EndOf:4] parse-type");

        trace!("[Start:5] parse-value");
        let value = value.map(Expression::parse_boxed).transpose()?;
        trace!("[EndOf:5] parse-value");

        trace!("[Start:5] construct-type-def");
        let assignment = Expression::Assignment { name, typ, value };
        trace!("[EndOf:5] construct-type-def");

        trace!("[EndOf] parse-field-definition");
        Ok(assignment)
    }
}
impl Parse for TypeDefinition {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-type-def");

        trace!("[Start:1] validate-rule");
        validate_rule!(line.as_rule(), type_definition);
        trace!("[EndOf:1] validate-rule");

        trace!("[Start:2] get-rules");
        let mut rules = line.into_inner();
        let name = next!(rules, "typedef(name)");
        let internals = rules.collect::<Vec<_>>();
        let mut fields = Vec::<Pair<Rule>>::new();
        let mut methods = Vec::<Pair<Rule>>::new();
        for internal in internals {
            match internal.as_rule() {
                Rule::function => methods.push(internal),
                Rule::field_definition => fields.push(internal),
                _ => unreachable!(),
            }
        }
        trace!("[EndOf:2] get-rules");

        trace!("[Start:3] parse-name");
        let name = Ident::parse(name)?;
        trace!("[EndOf:3] parse-name");

        trace!("[Start:4] parse-fields");
        let fields = fields
            .into_iter()
            .map(Self::parse_field_definition)
            .collect::<ParseResult<Vec<_>>>()?;
        trace!("[EndOf:4] parse-fields");

        trace!("[Start:5] parse-methods");
        let methods = methods
            .into_iter()
            .map(Function::parse)
            .collect::<ParseResult<Vec<_>>>()?;
        trace!("[EndOf:5] parse-methods");

        trace!("[Start:5] construct-type-def");
        let this = Self {
            name,
            fields,
            methods,
        };
        trace!("[EndOf:5] construct-type-def");

        trace!("[EndOf] parse-type-def");
        Ok(this)
    }
}
