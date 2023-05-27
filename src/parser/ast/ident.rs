use log::{error, trace};
use pest::iterators::Pair;

use crate::{
    parser::{
        ast::{context::ParseContext, span},
        error::{ParseError, ParseResult},
    },
    Rule,
};

use super::{Parse, Span};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ident {
    Identifier { name: String, span: Span },
    Type { name: String, span: Span },
    Native { name: String, span: Span },
}
impl ToString for Ident {
    fn to_string(&self) -> String {
        match self {
            Self::Identifier { name, .. } => name.clone(),
            Self::Type { name, .. } => name.clone(),
            Self::Native { name, .. } => name.clone(),
        }
    }
}
impl Ident {
    pub fn is_type(&self) -> bool {
        !matches!(self, Self::Identifier { .. })
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Identifier { span, .. } => span.clone(),
            Self::Type { span, .. } => span.clone(),
            Self::Native { span, .. } => span.clone(),
        }
    }

    pub fn parse_expect_type(line: Pair<Rule>) -> ParseResult<Self> {
        error!("[Start] parse-ident-expect-type");
        let this = Self::parse(line)?;
        if !this.is_type() {
            error!("[EndOf] parse-ident-expect-type: invalid ident kind");
            return Err(ParseError::ExpectedType {
                ident: this.to_string(),
                span: this.span(),
            });
        }
        error!("[EndOf] parse-ident-expect-type");
        Ok(this)
    }
}
impl Parse for Ident {
    fn parse(line: Pair<Rule>) -> ParseResult<Self> {
        trace!("[Start] parse-ident");

        let rule = line.as_rule();

        trace!("[Start:1] validate-rule");
        if matches!(rule, Rule::native) {
            trace!("[EndOf:1] validate-rule (native)");

            trace!("[Start:2] construct-native");
            let ident = line.as_str().to_owned();
            trace!("[EndOf:2] construct-native");

            trace!("[EndOf] parse-ident");
            Ok(Self::Native {
                name: ident,
                span: span(&line),
            })
        } else if matches!(rule, Rule::ident) {
            trace!("[EndOf:1] validate-rule (ident)");

            trace!("[Start:2] get-ident");
            let ident = line.as_str().to_owned();
            trace!("[EndOf:2] get-ident");

            trace!("[Start:2] get-kind");
            let type_information = ParseContext::is_type(&ident)?;
            let ident = match type_information {
                None => {
                    trace!("[EndOf:2] get-kind:Identifier");
                    Self::Identifier {
                        name: ident,
                        span: span(&line),
                    }
                }
                Some(info) if info.is_native => {
                    trace!("[EndOf:2] get-kind:Native");
                    Self::Native {
                        name: ident,
                        span: span(&line),
                    }
                }
                Some(_) => {
                    trace!("[EndOf:2] get-kind:Type");
                    Self::Type {
                        name: ident,
                        span: span(&line),
                    }
                }
            };
            trace!("[EndOf] parse-ident");

            Ok(ident)
        } else {
            trace!(
                "[EndOf] invalid-rule: Expected ident or native, got {:?}",
                rule
            );
            return ParseResult::Err(ParseError::InvalidRuleError {
                expected: Rule::ident,
                actual: rule,
            });
        }
    }
}
