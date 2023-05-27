use std::{
    collections::{HashMap, HashSet},
    sync::{Mutex, MutexGuard},
};

use log::warn;
use once_cell::sync::OnceCell;

use crate::parser::error::{context_uninitialized, unexpected_lock, ParseError, ParseResult};

use super::{ident::Ident, Span};

static INSTANCE: OnceCell<Mutex<ParseContext>> = OnceCell::new();

#[derive(Clone, Default)]
pub struct TypeInformation {
    pub is_native: bool,
    pub span: Span,
}
impl TypeInformation {
    pub fn native() -> Self {
        Self {
            is_native: true,
            span: Span::default(),
        }
    }
}

pub struct ParseContext {
    pub types: HashMap<String, TypeInformation>,
}
impl ParseContext {
    pub fn init() {
        let mut types = HashMap::new();
        types.insert("int".into(), TypeInformation::native());
        types.insert("float".into(), TypeInformation::native());
        types.insert("char".into(), TypeInformation::native());
        types.insert("str".into(), TypeInformation::native());
        let this = Self { types };

        if INSTANCE.set(Mutex::new(this)).is_err() {
            warn!("Attempt to re-initialize ParseContext.");
        }
    }

    pub fn get<'a>() -> ParseResult<MutexGuard<'a, Self>> {
        let mutex = INSTANCE.get().ok_or(context_uninitialized("add_type"))?;
        let this = mutex.try_lock().map_err(|_| unexpected_lock("add_type"))?;
        return Ok(this);
    }

    pub fn add_type(ident: String, info: TypeInformation) -> ParseResult<()> {
        let mut this = Self::get()?;

        if this.types.get(&ident).is_some() {
            return Err(ParseError::DuplicateType { ident });
        }

        this.types.insert(ident, info);

        Ok(())
    }

    pub fn is_type<S: ToString>(ident: S) -> ParseResult<Option<TypeInformation>> {
        let this = Self::get()?;
        return Ok(this.types.get(&ident.to_string()).cloned());
    }
}
