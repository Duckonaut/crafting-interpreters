use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Display, Formatter},
    rc::Rc,
};

use hezen_core::error::HezenError;

use crate::{
    ast::{Expr, Stmt},
    environment::{HezenEnvironment, HezenValue},
    function::HezenCallable,
    token::TokenType,
};

#[derive(Debug)]
pub(crate) enum HezenControl {
    Return(HezenValue),
    Break,
    Continue,
}

impl Display for HezenControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HezenControl::Return(value) => write!(f, "return {}", value),
            HezenControl::Break => write!(f, "break"),
            HezenControl::Continue => write!(f, "continue"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum HezenInterruption {
    Control(HezenControl),
    Error(HezenError),
}

#[derive(Debug)]
pub struct Interpreter {
    pub globals: Rc<HezenEnvironment>,
    environment: Rc<HezenEnvironment>,
    locals: HashMap<Expr, usize>,
}

macro_rules! binary_math_op {
    ($left:ident, $right:ident, $operator:ident, $op:tt, $result:tt) => {
        if let (HezenValue::Number(left), HezenValue::Number(right)) = ($left.clone(), $right.clone())
        {
            Ok(HezenValue::$result(left $op right))
        } else {
            Err(HezenError::runtime(
                $operator.position.file.clone(),
                $operator.position.line,
                $operator.position.column,
                format!(
                    "Operands must be two numbers, not '{}' and '{}'",
                    $left.type_name(),
                    $right.type_name()
                ),
            ))
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Rc::new(HezenEnvironment::default());

        Self {
            globals: globals.clone(),
            environment: globals,
            locals: HashMap::default(),
        }
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), HezenError> {
        for statement in statements {
            self.execute(statement).map_err(|i| match i {
                HezenInterruption::Control(c) => panic!("Uncaught {}, investigate resolver", c),
                HezenInterruption::Error(why) => why,
            })?;
        }

        Ok(())
    }

    pub(crate) fn execute(&mut self, stmt: &Stmt) -> Result<HezenValue, HezenInterruption> {
        match stmt {
            Stmt::Block(stmts) => self.execute_block(
                stmts.iter().collect(),
                Rc::new(HezenEnvironment::new(Some(self.environment.clone()))),
            ),
            Stmt::Class(_, _, _) => todo!(),
            Stmt::Expression(_) => todo!(),
            Stmt::Function(_, _, _) => todo!(),
            Stmt::If(_, _, _) => todo!(),
            Stmt::Var(_, _) => todo!(),
            Stmt::VarMut(_, _) => todo!(),
            Stmt::While(_, _) => todo!(),
            Stmt::Return(_, _) => todo!(),
            Stmt::Break => todo!(),
            Stmt::Continue => todo!(),
        }
    }

    pub(crate) fn evaluate(&mut self, expr: &Expr) -> Result<HezenValue, HezenError> {
        match expr {
            Expr::Assign(name, value) => {
                if self.locals.contains_key(expr) {
                    if self
                        .environment
                        .mutable_at(*self.locals.get(expr).unwrap(), &name)
                    {
                        let value = self.evaluate(value)?;
                        Rc::get_mut(&mut self.environment).unwrap().assign_at(
                            *self.locals.get(expr).unwrap(),
                            &name,
                            value,
                        )?;
                        Ok(HezenValue::Nil)
                    } else {
                        Err(HezenError::runtime(
                            name.position.file.clone(),
                            name.position.line,
                            name.position.column,
                            format!("Cannot assign to immutable variable '{}'", name.lexeme),
                        ))
                    }
                } else if self.globals.mutable(&name) {
                    let value = self.evaluate(value)?;
                    Rc::get_mut(&mut self.globals)
                        .unwrap()
                        .assign(&name, value)?;
                    Ok(HezenValue::Nil)
                } else {
                    Err(HezenError::runtime(
                        name.position.file.clone(),
                        name.position.line,
                        name.position.column,
                        format!("Cannot assign to immutable variable '{}'", name.lexeme),
                    ))
                }
            }
            Expr::Binary(left, operator, right) => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match operator.ty {
                    TokenType::Plus => {
                        if let (HezenValue::Number(left), HezenValue::Number(right)) = (left.clone(), right.clone())
                        {
                            Ok(HezenValue::Number(left + right))
                        } else if let (HezenValue::String(left), HezenValue::String(right)) =
                            (left.clone(), right.clone())
                        {
                            Ok(HezenValue::String(format!("{}{}", left, right)))
                        } else {
                            Err(HezenError::runtime(
                                operator.position.file.clone(),
                                operator.position.line,
                                operator.position.column,
                                format!(
                                    "Operands must be two numbers or two strings, not '{}' and '{}'",
                                    left.type_name(),
                                    right.type_name()
                                ),
                            ))
                        }
                    }
                    TokenType::Minus => {
                        binary_math_op!(left, right, operator, -, Number)
                    }
                    TokenType::Star => {
                        binary_math_op!(left, right, operator, *, Number)
                    }
                    TokenType::Slash => {
                        binary_math_op!(left, right, operator, /, Number)
                    }
                    TokenType::Greater => {
                        binary_math_op!(left, right, operator, >, Bool)
                    }
                    TokenType::GreaterEqual => {
                        binary_math_op!(left, right, operator, >=, Bool)
                    }
                    TokenType::Less => {
                        binary_math_op!(left, right, operator, <, Bool)
                    }
                    TokenType::LessEqual => {
                        binary_math_op!(left, right, operator, <=, Bool)
                    }
                    TokenType::EqualEqual => {
                        Ok(HezenValue::Bool(left == right))
                    }
                    TokenType::BangEqual => {
                        Ok(HezenValue::Bool(left != right))
                    }
                    _ => Err(HezenError::runtime(
                        operator.position.file.clone(),
                        operator.position.line,
                        operator.position.column,
                        format!("Invalid binary operator '{}'. Don't know how you did it, but that's a parser bug", operator.lexeme),
                    )),
                }
            }
            Expr::Call(callee, paren, args) => {
                let callee = self.evaluate(callee)?;

                let mut arguments = Vec::new();
                for arg in args {
                    arguments.push(self.evaluate(arg)?);
                }

                match callee {
                    HezenValue::Function(function) => {
                        if arguments.len() != function.arity() {
                            return Err(HezenError::runtime(
                                paren.position.file.clone(),
                                paren.position.line,
                                paren.position.column,
                                format!(
                                    "Expected {} arguments but got {}",
                                    function.arity(),
                                    arguments.len()
                                ),
                            ));
                        }

                        function.call(self, &arguments)
                    }
                    HezenValue::NativeFunction(function) => {
                        if arguments.len() != function.arity() {
                            return Err(HezenError::runtime(
                                paren.position.file.clone(),
                                paren.position.line,
                                paren.position.column,
                                format!(
                                    "Expected {} arguments but got {}",
                                    function.arity(),
                                    arguments.len()
                                ),
                            ));
                        }

                        function.call(self, &arguments)
                    }
                    HezenValue::Class(class) => {
                        if arguments.len() != class.arity() {
                            return Err(HezenError::runtime(
                                paren.position.file.clone(),
                                paren.position.line,
                                paren.position.column,
                                format!(
                                    "Expected {} arguments but got {}",
                                    class.arity(),
                                    arguments.len()
                                ),
                            ));
                        }

                        class.call(self, &arguments)
                    }
                    _ => Err(HezenError::runtime(
                        paren.position.file.clone(),
                        paren.position.line,
                        paren.position.column,
                        format!(
                            "Can only call functions and classes, not '{}'",
                            callee.type_name()
                        ),
                    )),
                }
            }
            Expr::Get(_, _) => todo!(),
            Expr::Grouping(_) => todo!(),
            Expr::Literal(_) => todo!(),
            Expr::Logical(_, _, _) => todo!(),
            Expr::Self_(_) => todo!(),
            Expr::Super(_, _) => todo!(),
            Expr::Set(_, _, _) => todo!(),
            Expr::Unary(_, _) => todo!(),
            Expr::Variable(_) => todo!(),

        }
    }

    pub(crate) fn execute_block(
        &mut self,
        stmts: Vec<&Stmt>,
        new_env: Rc<HezenEnvironment>,
    ) -> Result<HezenValue, HezenInterruption> {
        let prev = self.environment.clone();

        let mut value = HezenValue::Nil;

        self.environment = new_env;

        for stmt in stmts {
            value = match self.execute(stmt) {
                Ok(v) => v,
                Err(why) => {
                    self.environment = prev;

                    return Err(why);
                }
            }
        }

        self.environment = prev;

        Ok(value)
    }
}
