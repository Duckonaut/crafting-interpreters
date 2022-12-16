use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Get(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Self_(Token),
    Super(Token, Token),
    Set(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl std::hash::Hash for Literal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Bool(v) => v.hash(state),
            Self::Number(v) => v.to_bits().hash(state),
            Self::String(v) => v.hash(state),
            _ => {}
        }
    }
}

impl Eq for Literal {}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class(Token, Option<Expr>, Vec<Stmt>),
    Expression(Expr),
    Function(Token, Vec<Token>, Box<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Var(Token, Option<Expr>),
    VarMut(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
    Return(Token, Option<Expr>),
    Break,
    Continue,
}

macro_rules! wrap_expr {
    ($name:expr, $($expr:expr),*) => {
        wrap_expr_in_parentheses($name, vec![$($expr),*])
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Assign(name, value) => write!(
                f,
                "{}",
                wrap_expr!(&*format!("assign {} {}", name.lexeme, value),)
            ),
            Expr::Binary(left, op, right) => write!(
                f,
                "{}",
                wrap_expr!(&*format!("binary {} ", op.lexeme), Some(left), Some(right))
            ),
            Expr::Call(callee, _, arguments) => write!(
                f,
                "(call callee: {} {})",
                callee,
                wrap_expr_in_parentheses("arguments", arguments.iter().map(Some).collect())
            ),
            Expr::Get(object, name) => write!(f, "(get {} {})", object, name.lexeme),
            Expr::Grouping(expr) => write!(f, "{}", wrap_expr!("grouping", Some(expr))),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Logical(left, op, right) => write!(
                f,
                "{}",
                wrap_expr!(&*format!("logical {} ", op.lexeme), Some(left), Some(right))
            ),
            Expr::Self_(_) => write!(f, "self",),
            Expr::Super(keyword, method) => {
                write!(f, "(super {} {})", keyword.lexeme, method.lexeme)
            }
            Expr::Set(object, name, value) => {
                write!(f, "(set {} {} to {})", object, name.lexeme, value)
            }
            Expr::Unary(op, right) => write!(f, "{}", wrap_expr!(&*op.lexeme, Some(right))),
            Expr::Variable(name) => write!(f, "(variable {})", name.lexeme),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Nil => write!(f, "nil"),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block(statements) => write!(
                f,
                "{}",
                wrap_stmt_in_parentheses("block", statements.iter().collect())
            ),
            Stmt::Class(name, superclass, methods) => write!(
                f,
                "(class name: {}{} {})",
                name.lexeme,
                if let Some(superclass) = superclass {
                    format!(" superclass: {}", superclass)
                } else {
                    "".to_string()
                },
                wrap_stmt_in_parentheses("methods", methods.iter().collect())
            ),
            Stmt::Expression(expr) => write!(f, "{}", expr),
            Stmt::Function(name, params, body) => write!(
                f,
                "(function name: {}{} {})",
                name.lexeme,
                if !params.is_empty() {
                    format!(
                        " (params {})",
                        params
                            .iter()
                            .map(|p| p.lexeme.clone())
                            .collect::<Vec<String>>()
                            .join(" "),
                    )
                } else {
                    "".to_string()
                },
                body
            ),
            Stmt::If(condition, then_branch, else_branch) => write!(
                f,
                "(if {} (then {}) {})",
                condition,
                then_branch,
                if let Some(else_branch) = else_branch {
                    format!(" else {}", else_branch)
                } else {
                    "".to_string()
                }
            ),
            Stmt::Var(name, initializer) => write!(
                f,
                "(var {}{})",
                name.lexeme,
                if let Some(initializer) = initializer {
                    format!(" = {}", initializer)
                } else {
                    "".to_string()
                }
            ),
            Stmt::VarMut(name, initializer) => write!(
                f,
                "(var mut {}{})",
                name.lexeme,
                if let Some(initializer) = initializer {
                    format!(" = {}", initializer)
                } else {
                    "".to_string()
                }
            ),
            Stmt::While(condition, body) => write!(f, "(while {} {})", condition, body),
            Stmt::Return(_, value) => write!(
                f,
                "(return{})",
                if let Some(value) = value {
                    format!(" {}", value)
                } else {
                    "".to_string()
                }
            ),
            Stmt::Break => write!(f, "break"),
            Stmt::Continue => write!(f, "continue"),
        }
    }
}

fn wrap_expr_in_parentheses<'a>(name: impl Into<&'a str>, args: Vec<Option<&Expr>>) -> String {
    let mut result = String::new();
    result.push('(');
    result.push_str(name.into());

    for arg in args {
        result.push(' ');
        if let Some(arg) = arg {
            result.push_str(&arg.to_string());
        } else {
            result.push_str("nil");
        }
    }

    result.push(')');
    result
}

fn wrap_stmt_in_parentheses<'a>(name: impl Into<&'a str>, args: Vec<&Stmt>) -> String {
    let mut result = String::new();
    result.push('(');
    result.push_str(name.into());

    for arg in args {
        result.push(' ');
        result.push_str(&arg.to_string());
    }

    result.push(')');
    result
}
