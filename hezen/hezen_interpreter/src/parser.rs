use std::error::Error;
use std::fmt::{Display, Formatter};

use hezen_core::error::{HezenError, HezenErrorList};
use crate::ast::{Expr, Literal, Stmt};
use crate::token::{Token, TokenType, Tokens};

macro_rules! match_literal_token {
    ($self:ident, $ty:path) => {
        'block: {
            if $self.is_at_end() {
                break 'block None;
            }

            let token = $self.peek();

            if let $ty(v) = token.ty {
                $self.advance();
                Some($ty(v))
            } else {
                None
            }
        }
    };
}

type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Tokens,
    current: usize,
    errors: &'a mut HezenErrorList,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Tokens, errors: &'a mut HezenErrorList) -> Self {
        Self {
            tokens,
            current: 0,
            errors,
        }
    }

    pub fn parse(mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_token(TokenType::Class) {
            match self.class_declaration() {
                Ok(stmt) => return Some(stmt),
                Err(_) => self.synchronize(),
            }
        }

        if self.match_token(TokenType::Fn) {
            match self.function_declaration("function") {
                Ok(stmt) => return Some(stmt),
                Err(_) => self.synchronize(),
            }
        }

        if self.match_token(TokenType::Var) {
            match self.var_declaration() {
                Ok(stmt) => return Some(stmt),
                Err(_) => self.synchronize(),
            }
        }

        match self.statement() {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        let mutable = self.match_token(TokenType::Mut);

        let name = self.consume(TokenType::Identifier, "Expected variable name.")?;

        let initializer = if self.match_token(TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        if mutable {
            Ok(Stmt::VarMut(name, initializer))
        } else {
            Ok(Stmt::Var(name, initializer))
        }
    }

    fn function_declaration(&mut self, kind: &str) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier, &format!("Expected {} name.", kind))?;

        self.consume(
            TokenType::LeftParen,
            &format!("Expected '(' after {} name.", kind),
        )?;

        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(self.error(self.peek(), "Cannot have more than 255 parameters."));
                }

                parameters.push(self.consume(TokenType::Identifier, "Expected parameter name.")?);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters.")?;

        let body = self.block_statement()?;

        Ok(Stmt::Function(name, parameters, Box::new(body)))
    }

    fn class_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expected class name.")?;

        let superclass = if self.match_token(TokenType::Less) {
            Some(Expr::Variable(self.consume(
                TokenType::Identifier,
                "Expected superclass name.",
            )?))
        } else {
            None
        };

        self.consume(TokenType::LeftBrace, "Expected '{' before class body.")?;

        let mut methods = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function_declaration("method")?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after class body.")?;

        Ok(Stmt::Class(name, superclass, methods))
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        if self.match_token(TokenType::Return) {
            return self.return_statement();
        } else if self.match_token(TokenType::Break) {
            return self.break_statement();
        } else if self.match_token(TokenType::Continue) {
            return self.continue_statement();
        } else if self.match_token(TokenType::For) {
            return self.for_statement();
        } else if self.match_token(TokenType::If) {
            return self.if_statement();
        } else if self.match_token(TokenType::While) {
            return self.while_statement();
        } else if self.check(TokenType::LeftBrace) {
            return self.block_statement();
        }

        self.expression_statement()
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;

        Ok(Stmt::Expression(expr))
    }

    fn block_statement(&mut self) -> ParseResult<Stmt> {
        let mut statements = Vec::new();

        self.consume(TokenType::LeftBrace, "Expected '{' before block.")?;

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block.")?;

        Ok(Stmt::Block(statements))
    }

    fn if_statement(&mut self) -> ParseResult<Stmt> {
        let condition = self.expression()?;

        let then_branch = self.statement()?;

        let else_branch = if self.match_token(TokenType::Else) {
            if self.match_token(TokenType::If) {
                Some(Box::new(self.if_statement()?))
            } else {
                Some(Box::new(self.block_statement()?))
            }
        } else {
            None
        };

        Ok(Stmt::If(condition, Box::new(then_branch), else_branch))
    }

    fn while_statement(&mut self) -> ParseResult<Stmt> {
        let condition = self.expression()?;

        let then = self.block_statement()?;

        Ok(Stmt::While(condition, Box::new(then)))
    }

    fn for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'.")?;

        let initializer = if self.match_token(TokenType::Semicolon) {
            None
        } else if self.match_token(TokenType::Var) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;

        let mut body = self.block_statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(increment)]);
        }

        if let Some(condition) = condition {
            body = Stmt::While(condition, Box::new(body));
        } else {
            body = Stmt::While(Expr::Literal(Literal::Bool(true)), Box::new(body));
        }

        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![initializer, body]);
        }

        Ok(body)
    }

    fn break_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after 'break' statement.",
        )?;
        Ok(Stmt::Break)
    }

    fn continue_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after continue statement.",
        )?;
        Ok(Stmt::Continue)
    }

    fn return_statement(&mut self) -> ParseResult<Stmt> {
        let keyword = self.previous();

        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after return value.")?;

        Ok(Stmt::Return(keyword, value))
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.or()?;

        if self.match_token(TokenType::Equal) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(name) => return Ok(Expr::Assign(name, Box::new(value))),
                Expr::Get(object, name) => return Ok(Expr::Set(object, name, Box::new(value))),
                _ => {}
            }

            return Err(self.error(equals, "Invalid assignment target."));
        }

        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.and()?;

        while self.match_token(TokenType::Or) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;

        while self.match_token(TokenType::And) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token(TokenType::BangEqual) || self.match_token(TokenType::EqualEqual) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;

        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        while self.match_token(TokenType::Star) || self.match_token(TokenType::Slash) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.match_token(TokenType::Bang) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        if self.match_token(TokenType::Minus) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.call()
    }

    fn call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(TokenType::Dot) {
                let name =
                    self.consume(TokenType::Identifier, "Expected property name after '.'.")?;
                expr = Expr::Get(Box::new(expr), name);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> ParseResult<Expr> {
        let mut args = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if args.len() >= 255 {
                    return Err(self.error(self.peek(), "Cannot have more than 255 arguments."));
                }

                args.push(self.expression()?);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        Ok(Expr::Call(Box::new(callee), paren, args))
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        if self.match_token(TokenType::False) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.match_token(TokenType::True) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.match_token(TokenType::Nil) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if let Some(TokenType::Number(n)) = match_literal_token!(self, TokenType::Number) {
            return Ok(Expr::Literal(Literal::Number(n)));
        }

        if let Some(TokenType::String(s)) = match_literal_token!(self, TokenType::String) {
            return Ok(Expr::Literal(Literal::String(s)));
        }

        if self.match_token(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.match_token(TokenType::Self_) {
            return Ok(Expr::Self_(self.previous()));
        }

        if self.match_token(TokenType::Super) {
            let keyword = self.previous();
            self.consume(TokenType::Dot, "Expected '.' after 'super'.")?;
            let method = self.consume(TokenType::Identifier, "Expected superclass method name.")?;
            return Ok(Expr::Super(keyword, method));
        }

        if self.match_token(TokenType::Identifier) {
            return Ok(Expr::Variable(self.previous()));
        }

        Err(self.error(self.peek(), "Expected expression."))
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ty == token_type
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ty == TokenType::Semicolon {
                return;
            }

            match self.peek().ty {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).clone()
    }

    fn error(&mut self, token: Token, message: &str) -> ParseError {
        let error = ParseError::new(token.clone(), message.into());

        self.errors.add(HezenError::syntax_error(
            token.position.file,
            token.position.line,
            token.position.column,
            message.into(),
        ));

        error
    }
}

#[derive(Debug)]
pub struct ParseError {
    token: Token,
    message: String,
}

impl ParseError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{} line {}:{}] Error at '{}': {}",
            self.token.position.file,
            self.token.position.line,
            self.token.position.column,
            self.token.lexeme,
            self.message
        )
    }
}

impl Error for ParseError {}
