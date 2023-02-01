use std::fmt::Display;

use hezen_core::error::HezenLineInfo;

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    String(String),
    Number(f64),

    // keywords.
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    Self_,
    True,
    Var,
    While,
    Mut,
    Break,
    Continue,

    Eof,
    Builtin,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl std::hash::Hash for TokenType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::String(v) => v.hash(state),
            Self::Number(v) => v.to_bits().hash(state),
            _ => {}
        }
    }
}

impl Eq for TokenType {
    
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Identifier => write!(f, "identifier"),
            TokenType::String(s) => write!(f, "\"{s}\""),
            TokenType::Number(n) => write!(f, "{n}"),
            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::Fn => write!(f, "fn"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::Self_ => write!(f, "self"),
            TokenType::True => write!(f, "true"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),
            TokenType::Mut => write!(f, "mut"),
            TokenType::Break => write!(f, "break"),
            TokenType::Continue => write!(f, "continue"),
            TokenType::Eof => write!(f, "eof"),
            TokenType::Builtin => write!(f, "builtin"),
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub position: HezenLineInfo,
}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.lexeme.hash(state);
        self.position.hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.lexeme == other.lexeme && self.position == other.position
    }
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, position: HezenLineInfo) -> Self {
        Self {
            ty,
            lexeme,
            position,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.lexeme, self.ty)
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub list: Vec<Token>,
}

impl Tokens {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, token: Token) {
        self.list.push(token);
    }

    pub fn get(&self, index: usize) -> &Token {
        &self.list[index]
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.list.iter() {
            writeln!(f, "{token}")?;
        }

        Ok(())
    }
}
