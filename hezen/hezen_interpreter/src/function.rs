use crate::{token::Token, ast::Stmt, environment::HezenEnvironment};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Clone)]
pub struct HezenFunction {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Stmt,
    closure: HezenEnvironment,
    initializer: bool,
}
