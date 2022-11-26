use once_cell::sync::Lazy;

use crate::{
    error::{HezenError, HezenErrorList, HezenLineInfo},
    token::{Token, TokenType, Tokens},
};

static NEWLINE_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"(^|[^\\])(\\n)").unwrap());

static TAB_REGEX: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"(^|[^\\])(\\t)").unwrap());

static BACKSLASH_REGEX: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"(\\\\)").unwrap());

pub struct Lexer<'a> {
    filename: String,
    source: String,
    tokens: Tokens,
    start: usize,
    current: usize,
    line: usize,
    line_start: usize,
    errors: &'a mut HezenErrorList,
}

impl<'a> Lexer<'a> {
    pub fn new(filename: String, source: String, errors: &'a mut HezenErrorList) -> Self {
        Self {
            filename,
            source,
            tokens: Tokens::new(),
            start: 0,
            current: 0,
            line: 1,
            line_start: 0,
            errors,
        }
    }

    pub fn get_tokens(mut self) -> Tokens {
        while !self.is_at_end() {
            self.start = self.current;
            self.get_token();
        }

        self.tokens.add(Token::new(
            TokenType::Eof,
            "".to_string(),
            HezenLineInfo::new(self.filename.clone(), self.line, self.line_start),
        ));

        self.tokens
    }

    fn get_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.try_match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.try_match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.try_match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.try_match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.try_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.line_start = self.current;
            }
            '"' => self.handle_string(),
            '0'..='9' => self.handle_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.handle_identifier(),
            _ => {
                self.error("Unexpected character.");
            }
        }
    }

    fn add_token(&mut self, token: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.add(Token::new(
            token,
            text,
            HezenLineInfo::new(self.filename.clone(), self.line, self.line_start),
        ));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn try_match(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.line_start = self.current;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();

        let value = NEWLINE_REGEX.replace_all(&value, "$1\n");
        let value = TAB_REGEX.replace_all(&value, "$1\t");
        let value = BACKSLASH_REGEX.replace_all(&value, "$1\\");

        self.add_token(TokenType::String(value.to_string()));
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];
        self.add_token(TokenType::Number(value.parse().unwrap()));
    }

    fn handle_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let token = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fn" => TokenType::Fn,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "self" => TokenType::Self_,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "mut" => TokenType::Mut,
            "while" => TokenType::While,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            _ => TokenType::Identifier,
        };

        self.add_token(token);
    }

    fn error(&mut self, message: &str) {
        self.errors.add(HezenError::syntax_error(
            message.to_string(),
            HezenLineInfo::new(self.filename.clone(), self.line, self.current - self.line_start),
        ));
    }
}
