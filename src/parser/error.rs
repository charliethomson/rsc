use std::num::ParseFloatError;

use snailquote::UnescapeError;
use thiserror::Error;

use crate::Rule;

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
}

pub fn missing(slug: &'static str) -> ParseError {
    ParseError::MissingItem { slug }
}

pub type ParseResult<T> = Result<T, ParseError>;
