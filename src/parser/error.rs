use std::num::ParseFloatError;

use snailquote::UnescapeError;
use thiserror::Error;

use crate::Rule;

use super::ast::{ident::Ident, Span};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid rule. Expected {expected:?} got {actual:?}")]
    InvalidRuleError { expected: Rule, actual: Rule },
    #[error("Invalid rule. Expected {expected:?} got {actual:?}")]
    InvalidRuleErrorOneOf { expected: Vec<Rule>, actual: Rule },
    #[error("Missing item. {slug}")]
    MissingItem { slug: &'static str },
    #[error("Error parsing float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Invalid character literal: {reason}")]
    InvalidCharError { reason: &'static str },
    #[error("Failed to unescape string")]
    UnescapeError(#[from] UnescapeError),
    #[error("General context error: {message} ({origin})")]
    ContextError {
        message: &'static str,
        origin: &'static str,
    },
    #[error("Duplicate type definition")]
    DuplicateType { ident: String },
    #[error("Failed to convert from string")]
    FromStrError {
        reason: String,
        origin: &'static str,
    },
    #[error("Expected type, got identifier")]
    ExpectedType { ident: String, span: Span },
}

pub fn missing(slug: &'static str) -> ParseError {
    ParseError::MissingItem { slug }
}
pub fn context_uninitialized(origin: &'static str) -> ParseError {
    ParseError::ContextError {
        message: "Context uninitialized",
        origin,
    }
}
pub fn unexpected_lock(origin: &'static str) -> ParseError {
    ParseError::ContextError {
        message: "Context locked - this shouldn't be possible.",
        origin,
    }
}
pub fn bad_fromstr(reason: String, origin: &'static str) -> ParseError {
    ParseError::FromStrError { reason, origin }
}

pub type ParseResult<T> = Result<T, ParseError>;
