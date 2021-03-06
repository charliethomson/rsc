lazy_static::lazy_static! {
    pub static ref FLOAT_RE: Regex = Regex::new(r"[-+]?[0-9]*\.[0-9]+([eE][-+]?[0-9]+)?").unwrap();
    pub static ref STRING_RE: Regex = Regex::new(r#""(?:[^"\\]|\\.)*""#).unwrap();
    pub static ref CHARS_RE: Regex = Regex::new(r#"'(?:[^"\\]|\\.)*'"#).unwrap();
}

use std::collections::{HashMap, VecDeque};

use regex::Regex;

use crate::lexer::tok::{Keyword, Operator};

use self::tok::Whitespace;
pub use self::{
    err::LexicalError,
    pos::{Location, Spanned},
    tok::Token,
};

pub mod err;
pub mod pos;
pub mod tok;

fn first_rest<T: Clone>(v: &Vec<T>) -> Option<(T, Vec<T>)> {
    let vector = v.clone();
    if vector.len() == 0 {
        return None;
    }

    let mut iter = vector.into_iter();

    // Unwrap is safe, length check at the top
    let first = iter.next().unwrap();

    let rest = iter.collect::<Vec<T>>();

    Some((first, rest))
}

fn is_ident_start(ch: &char) -> bool {
    match ch {
        '_' | 'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}
fn is_ident_continue(ch: &char) -> bool {
    match ch {
        '0'..='9' => true,
        ch => is_ident_start(ch),
    }
}

fn is_digit(ch: &char) -> bool {
    ch.is_digit(10)
}

fn is_hex(ch: &char) -> bool {
    ch.is_digit(16)
}

fn is_float(s: &String) -> bool {
    FLOAT_RE.is_match(s)
}

fn is_operator_char(ch: &char) -> bool {
    "-+&.>/^|<*!%=`?".contains(&ch.to_string())
}

fn is_lexical_whitespace(ch: &char) -> bool {
    "\r\n".contains(&ch.to_string())
}

fn is_kw_char(ch: &char) -> bool {
    "chnfuaewirtsylpo".contains(&ch.to_string())
}

fn is_keyword(s: &String) -> bool {
    [
        "if", "else", "while", "for", "return", "type", "static", "oneof", "when", "in", "nil",
        "false", "true",
    ]
    .contains(&s.as_str())
}

fn is_builtin_type(s: &String) -> bool {
    ["number", "string", "bool", "nil"].contains(&s.as_str())
}

fn is_punctuation(ch: &char) -> bool {
    ",;:{}()[]".contains(&ch.to_string())
}

pub struct Lexer {
    text: String,
    location: Location,
    seen_first: bool,
    last: Option<Token>,
    types: Vec<String>,
    definitions: HashMap<String, Vec<Spanned>>,
    token_buffer: VecDeque<Spanned>,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            text: input.to_string(),
            location: Location {
                line: 0.into(),
                column: 0.into(),
                absolute: 0.into(),
            },
            seen_first: false,
            last: None,
            types: vec![],
            definitions: HashMap::new(),
            token_buffer: VecDeque::new(),
        }
    }

    fn current(&self) -> Option<char> {
        self.lookahead_offset(0)
    }

    fn lookahead(&self) -> Option<char> {
        self.lookahead_offset(1)
    }
    fn lookahead_offset(&self, offset: usize) -> Option<char> {
        self.text
            .chars()
            .nth(self.location.absolute.to_usize() + offset)
    }

    fn lookahead_eq(&self, ch: char, offset: usize) -> bool {
        self.lookahead_offset(offset)
            .map(|next| next == ch)
            .unwrap_or_default()
    }

    fn operator(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let operators = vec![
            "`", ">>", "<<", "*", "/", "%", "^", "|", "+", "++", "-", "--", "<", ">", "<=", ">=",
            "==", "!=", "&&", "^^", "||", "=", "<<=", ">>=", "*=", "/=", "&=", "|=", "^=", "+=",
            "-=", "!", "&", "->", "=>", "?", ".",
        ];

        let mut buffer = ch.to_string();
        let mut i = 0;
        while let Some(ch) = self.lookahead_offset(i) {
            if !is_operator_char(&ch) {
                break;
            } else {
                buffer.push(ch);
            }
            i += 1;
        }

        self.location = self.location.add(buffer.len() - 1);
        if operators.contains(&buffer.as_str()) {
            Some(Spanned {
                start,
                tok: Token::Operator(Operator::from(buffer)),
                end: self.location,
            })
        } else {
            None
        }
    }

    fn number(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut s = self
            .text
            .chars()
            .skip(self.location.absolute.to_usize())
            .take_while(|ch| is_digit(ch) || "eE+-.".contains(&ch.to_string()))
            .fold(ch.to_string(), |mut s, ch| {
                s.push(ch);
                s
            });

        if (s.ends_with(vec!['-', '+'].as_slice())) {
            s.pop();
        }

        self.location = self.location.add(s.len() - 1);

        if is_float(&s) {
            Some(Spanned {
                start,
                end: self.location,
                tok: Token::Float(s.parse::<f64>().unwrap()),
            })
        } else if s.starts_with(vec!['-', '+'].as_slice()) {
            Some(Spanned {
                start,
                end: self.location,
                tok: Token::Signed(s.parse::<i64>().unwrap()),
            })
        } else {
            Some(Spanned {
                start,
                end: self.location,
                tok: Token::Unsigned(s.parse::<u64>().unwrap()),
            })
        }
    }

    fn identifier(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut ident_str = ch.to_string();

        ident_str = self
            .text
            .chars()
            .skip(self.location.absolute.to_usize())
            .take_while(is_ident_continue)
            .fold(ident_str, |mut ident_str, ch| {
                ident_str.push(ch);
                ident_str
            });

        self.location = self.location.add(ident_str.len() - 1);

        if is_keyword(&ident_str) {
            Some(Spanned {
                start,
                tok: Token::Keyword(Keyword::from(ident_str)),
                end: self.location,
            })
        } else if is_builtin_type(&ident_str) {
            Some(Spanned {
                start,
                tok: Token::TypeIdent(ident_str),
                end: self.location,
            })
        } else if self.definitions.contains_key(&ident_str) {
            if let Some((first, rest)) = first_rest(&self.definitions.get(&ident_str).unwrap()) {
                for spanned in rest.into_iter() {
                    self.token_buffer.push_back(spanned)
                }

                Some(first)
            } else {
                None
            }
        } else {
            Some(Spanned {
                start,
                tok: if !self.types.contains(&ident_str) {
                    Token::Identifier(ident_str)
                } else {
                    Token::TypeIdent(ident_str)
                },
                end: self.location,
            })
        }
    }

    fn punct(&mut self, start: Location, ch: char) -> Option<Spanned> {
        match ch {
            ',' => Some(Spanned {
                start,
                tok: Token::Comma,
                end: start.add(1),
            }),
            ';' => Some(Spanned {
                start,
                tok: Token::Semicolon,
                end: start.add(1),
            }),
            ':' => Some(Spanned {
                start,
                tok: Token::Colon,
                end: start.add(1),
            }),
            '{' => Some(Spanned {
                start,
                tok: Token::OpenCurly,
                end: start.add(1),
            }),
            '}' => Some(Spanned {
                start,
                tok: Token::CloseCurly,
                end: start.add(1),
            }),
            '(' => Some(Spanned {
                start,
                tok: Token::OpenParen,
                end: start.add(1),
            }),
            ')' => Some(Spanned {
                start,
                tok: Token::CloseParen,
                end: start.add(1),
            }),
            '[' => Some(Spanned {
                start,
                tok: Token::OpenSquare,
                end: start.add(1),
            }),
            ']' => Some(Spanned {
                start,
                tok: Token::CloseSquare,
                end: start.add(1),
            }),
            _ => None,
        }
    }

    fn string_literal(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut s = ch.to_string();
        for ch in self.text.chars().skip(self.location.absolute.to_usize()) {
            self.location = self.location.shift(ch);
            s.push(ch);
            if STRING_RE.is_match(&s) || CHARS_RE.is_match(&s) {
                break;
            }
        }

        if (STRING_RE.is_match(&s)) {
            Some(Spanned {
                start,
                tok: Token::String((&s).trim_matches('"').to_owned()),
                end: self.location,
            })
        } else if CHARS_RE.is_match(&s) {
            Some(Spanned {
                start,
                tok: Token::String((&s).trim_matches('\'').to_owned()),
                end: self.location,
            })
        } else {
            None
        }
    }
    fn handle_define(&mut self, start: Location) -> Option<()> {
        //                    #define A 5
        // `start` is right here     ^
        let mut buffer = String::new();
        while let Some((_, ch)) = self.bump() {
            if ch == '\n' {
                break;
            } else {
                buffer.push(ch)
            }
        }
        buffer = buffer.trim().into();

        let mut idx = 0;
        let ident = buffer
            .chars()
            .take_while(|ch| {
                idx += 1;
                !ch.is_whitespace()
            })
            .collect::<String>();
        let rest = buffer.chars().skip(idx).collect::<String>();
        let rest = rest.trim();

        let mut lexer = Lexer::new(rest);

        let mut toks = vec![];

        while let Some(Ok((start, tok, end))) = lexer.next() {
            toks.push(Spanned { start, tok, end })
        }

        println!("Encountered definition: {:?} = {:?}", ident, rest);
        self.definitions.insert(ident, toks);

        Some(())
    }

    fn directive(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut s = self
            .text
            .chars()
            .skip(self.location.absolute.to_usize())
            .take_while(char::is_ascii_lowercase)
            .fold(String::new(), |mut s, ch| {
                s.push(ch);
                s
            });

        self.location = self.location.add(s.len());

        self.handle_define(self.location)?;

        Some(Spanned::nop(start, self.location))
    }

    fn comment_single_line(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut comment = ch.to_string();
        while let Some((loc, ch)) = self.bump() {
            if ch == '\n' {
                return Some(Spanned {
                    start,
                    tok: Token::SingleLineComment(comment),
                    end: loc,
                });
            }
            comment.push(ch);
        }
        None
    }
    fn comment_multi_line(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut comment = ch.to_string();
        let mut possible_end = false;
        while let Some((loc, ch)) = self.bump() {
            comment.push(ch);

            if possible_end {
                if ch == '/' {
                    return Some(Spanned {
                        start,
                        tok: Token::MultiLineComment(comment),
                        end: loc,
                    });
                }

                possible_end = false
            }

            if ch == '*' {
                possible_end = true
            }
        }
        None
    }

    fn bump(&mut self) -> Option<(Location, char)> {
        match self.current() {
            Some(ch) => {
                self.location = self.location.shift(ch);
                Some((self.location, ch))
            }
            None => None,
        }
    }

    fn lexical_whitespace(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut white = ch.to_string();
        let mut i = 0;
        while let Some(ch) = self.lookahead_offset(i) {
            if !is_lexical_whitespace(&ch) {
                return Some(Spanned {
                    start,
                    tok: Token::Whitespace(Whitespace::from(
                        Regex::new("[\n]+")
                            .unwrap()
                            .replace(&white, "\n")
                            .to_string(),
                    )),
                    end: self.location,
                });
            }

            white.push(ch);
            i += 1;
        }

        None
    }
}
impl Iterator for Lexer {
    type Item = Result<FlattenedSpanned, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.token_buffer.is_empty() {
            return Ok(self.token_buffer.pop_front().map(Spanned::into_flattened)).transpose();
        }
        while let Some((start, ch)) = self.bump() {
            let result = match ch {
                ch if ch == '/' && self.lookahead_eq('/', 0) => {
                    self.comment_single_line(start, ch).map(Result::Ok)
                }
                ch if ch == '/' && self.lookahead_eq('*', 0) => {
                    self.comment_multi_line(start, ch).map(Result::Ok)
                }
                ch if is_operator_char(&ch) => Some(
                    self.operator(start, ch)
                        .ok_or(LexicalError::ExpectedOperator(start, ch)),
                ),
                ch if is_digit(&ch) || "+-".contains(ch) => Some(
                    self.number(start, ch)
                        .ok_or(LexicalError::InvalidDigit(start)),
                ),
                ch if is_lexical_whitespace(&ch) => {
                    self.lexical_whitespace(start, ch).map(Result::Ok)
                }
                ch if ch.is_whitespace() => continue,
                ch if is_ident_start(&ch) => Some(
                    self.identifier(start, ch)
                        .ok_or(LexicalError::ExpectedIdentOrKw(start)),
                ),
                ch if is_punctuation(&ch) => Some(
                    self.punct(start, ch)
                        .ok_or(LexicalError::ExpectedPunctuation(start)),
                ),
                '"' | '\'' => Some(
                    self.string_literal(start, ch)
                        .ok_or(LexicalError::InvalidString(start)),
                ),
                '#' => Some(
                    self.directive(start, ch)
                        .ok_or(LexicalError::InvalidDirective(start)),
                ),
                _ => None,
            }
            .map(|res| res.map(Spanned::into_flattened));

            if let Some(Ok((_, tok, _))) = result.clone() {
                self.last = Some(tok)
            } else {
                self.last = None
            }

            return result;
        }

        // println!("Definitions: {:#?}", self.definitions);
        None
    }
}

pub type FlattenedSpanned = (Location, Token, Location);
impl Spanned {
    fn into_flattened(self) -> FlattenedSpanned {
        (self.start, self.tok, self.end)
    }
}
