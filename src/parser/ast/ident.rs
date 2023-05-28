use log::{error, trace};
use pest::iterators::Pair;

use crate::{
    parser::{
        ast::{context::ParseContext, span},
        error::{bad_fromstr, ParseError, ParseResult},
    },
    Rule,
};

use super::{Parse, Span};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReservedIdent {
    Slf,
}
impl ReservedIdent {
    pub fn is_reserved<S: ToString>(s: S) -> bool {
        Self::try_from(s.to_string()).is_ok()
    }
}
impl TryFrom<String> for ReservedIdent {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "self" => Ok(Self::Slf),
            _ => Err(bad_fromstr(value, "reserved-ident-tryfrom")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ident {
    Identifier {
        name: String,
        span: Span,
    },
    Type {
        name: String,
        span: Span,
    },
    Native {
        name: String,
        span: Span,
    },
    Reserved {
        name: String,
        span: Span,
        ident: ReservedIdent,
    },
}
impl ToString for Ident {
    fn to_string(&self) -> String {
        match self {
            Self::Identifier { name, .. } => name.clone(),
            Self::Type { name, .. } => name.clone(),
            Self::Native { name, .. } => name.clone(),
            Self::Reserved { name, .. } => name.clone(),
        }
    }
}
impl Ident {
    fn ident(name: String, span: Span) -> ParseResult<Self> {
        if ReservedIdent::is_reserved(&name) {
            Ok(Self::Reserved {
                name: name.clone(),
                span,
                ident: ReservedIdent::try_from(name)?,
            })
        } else {
            Ok(Self::Identifier { name, span })
        }
    }
}
impl Ident {
    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type { .. }) || matches!(self, Self::Native { .. })
    }
    pub fn is_reserved(&self) -> bool {
        !matches!(self, Self::Reserved { .. })
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Identifier { span, .. } => span.clone(),
            Self::Type { span, .. } => span.clone(),
            Self::Native { span, .. } => span.clone(),
            Self::Reserved { span, .. } => span.clone(),
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
        } else if matches!(rule, Rule::ID_anon) {
            trace!("[EndOf:1] validate-rule (ID_anon)");

            trace!("[Start:2] construct-anonymous");
            let ident = line.as_str().to_owned();
            trace!("[EndOf:2] construct-anonymous");

            trace!("[EndOf] parse-ident");
            Ok(Self::ident(ident, span(&line))?)
        } else if matches!(rule, Rule::ident) {
            trace!("[EndOf:1] validate-rule (ident)");

            trace!("[Start:2] get-name");
            let name = line.as_str().to_owned();
            trace!("[EndOf:2] get-name");

            trace!("[Start:2] get-kind");
            let type_information = ParseContext::is_type(&name)?;
            let name = match type_information {
                None => {
                    trace!("[EndOf:2] get-kind:Identifier");
                    Self::ident(name, span(&line))?
                }
                Some(info) if info.is_native => {
                    trace!("[EndOf:2] get-kind:Native");
                    Self::Native {
                        name,
                        span: span(&line),
                    }
                }
                Some(_) => {
                    trace!("[EndOf:2] get-kind:Type");
                    Self::Type {
                        name,
                        span: span(&line),
                    }
                }
            };
            trace!("[EndOf] parse-ident");

            Ok(name)
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
