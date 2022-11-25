use once_cell::sync::Lazy;

static NEWLINE_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"(^|[^\\])(\\n)").unwrap());

static TAB_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"(^|[^\\])(\\t)").unwrap());

static BACKSLASH_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"(\\\\)").unwrap());

pub enum Token {
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
    String,
    Number,

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

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    line_start: usize,
}
