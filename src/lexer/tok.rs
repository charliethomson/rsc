#[derive(Debug, Clone)]
pub enum Token {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Identifier(String),
    TypeIdent(String),
    Operator(Operator),
    Keyword(Keyword),
    SingleLineComment(String),
    MultiLineComment(String),
    Whitespace(Whitespace),
    NOP,
    Comma,
    Semicolon,
    Colon,
    OpenCurly,
    CloseCurly,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
}
impl Token {
    pub fn expect_operator(self) -> Operator {
        match self {
            Self::Operator(op) => op,
            _ => panic!("Attempt to get operator from non operator token."),
        }
    }
    pub fn expect_keyword(self) -> Keyword {
        match self {
            Self::Keyword(kw) => kw,
            _ => panic!("Attempt to get keyword from non keyword token."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Whitespace {
    Newline,
}
impl From<String> for Whitespace {
    fn from(s: String) -> Self {
        match s.as_str() {
            "\n" | "\r\n" => Self::Newline,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    If,
    Else,
    While,
    For,
    Return,
    Type,
    Static,
    OneOf,
    When,
    In,
    Nil,
    False,
    True,
}
impl From<String> for Keyword {
    fn from(s: String) -> Self {
        match s.as_str() {
            "if" => Self::If,
            "else" => Self::Else,
            "while" => Self::While,
            "for" => Self::For,
            "return" => Self::Return,
            "type" => Self::Type,
            "static" => Self::Static,
            "oneof" => Self::OneOf,
            "when" => Self::When,
            "in" => Self::In,
            "nil" => Self::Nil,
            "false" => Self::False,
            "true" => Self::True,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Power,
    Shr,
    Shl,
    Star,
    Div,
    Mod,
    BXor,
    BOr,
    Add,
    Inc,
    Sub,
    Dec,
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
    And,
    Xor,
    Or,
    Assign,
    AShl,
    AShr,
    AMul,
    ADiv,
    ABAnd,
    ABOr,
    ABXor,
    AAdd,
    ASub,
    Not,
    Amp,
    ThinArrow,
    ThickArrow,
    Optional,
    MemberAccess,
}
impl From<String> for Operator {
    fn from(s: String) -> Self {
        match s.as_str() {
            "`" => Self::Power,
            ">>" => Self::Shr,
            "<<" => Self::Shl,
            "*" => Self::Star,
            "/" => Self::Div,
            "%" => Self::Mod,
            "^" => Self::BXor,
            "|" => Self::BOr,
            "+" => Self::Add,
            "++" => Self::Inc,
            "-" => Self::Sub,
            "--" => Self::Dec,
            "<" => Self::Lt,
            ">" => Self::Gt,
            "<=" => Self::Le,
            ">=" => Self::Ge,
            "==" => Self::Eq,
            "!=" => Self::Ne,
            "&&" => Self::And,
            "^^" => Self::Xor,
            "||" => Self::Or,
            "=" => Self::Assign,
            "<<=" => Self::AShl,
            ">>=" => Self::AShr,
            "*=" => Self::AMul,
            "/=" => Self::ADiv,
            "&=" => Self::ABAnd,
            "|=" => Self::ABOr,
            "^=" => Self::ABXor,
            "+=" => Self::AAdd,
            "-=" => Self::ASub,
            "!" => Self::Not,
            "&" => Self::Amp,
            "->" => Self::ThinArrow,
            "=>" => Self::ThickArrow,
            "?" => Self::Optional,
            "." => Self::MemberAccess,
            _ => unreachable!(),
        }
    }
}
