use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    rc::Rc,
};

use hezen_core::error::{HezenError, HezenLineInfo};

use crate::{
    ast::{Expr, Stmt},
    class::HezenClass,
    environment::{HezenEnvironmentHandle, HezenValue},
    function::{HezenCallable, HezenFunction, HezenNativeFunction},
    token::{Token, TokenType},
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
            HezenControl::Return(value) => write!(f, "return {value}"),
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
    pub globals: HezenEnvironmentHandle,
    environment: HezenEnvironmentHandle,
    pub locals: HashMap<Token, usize>,
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
        let mut globals = HezenEnvironmentHandle::default();

        globals.define(
            Token::new(
                TokenType::Builtin,
                "clock".to_string(),
                HezenLineInfo {
                    line: 0,
                    column: 0,
                    file: "<builtin>".to_string(),
                },
            ),
            HezenValue::NativeFunction(Rc::new(HezenNativeFunction::new(
                Token::new(
                    TokenType::Builtin,
                    "clock".to_string(),
                    HezenLineInfo {
                        line: 0,
                        column: 0,
                        file: "<builtin>".to_string(),
                    },
                ),
                0,
                |_| {
                    Ok(HezenValue::Number(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs_f64(),
                    ))
                },
            ))),
            false,
        );

        globals.define(
            Token::new(
                TokenType::Builtin,
                "print".to_string(),
                HezenLineInfo {
                    line: 0,
                    column: 0,
                    file: "<builtin>".to_string(),
                },
            ),
            HezenValue::NativeFunction(Rc::new(HezenNativeFunction::new(
                Token::new(
                    TokenType::Builtin,
                    "print".to_string(),
                    HezenLineInfo {
                        line: 0,
                        column: 0,
                        file: "<builtin>".to_string(),
                    },
                ),
                1,
                |args| {
                    print!("{}", args[0]);
                    Ok(HezenValue::Nil)
                },
            ))),
            false,
        );

        globals.define(
            Token::new(
                TokenType::Builtin,
                "println".to_string(),
                HezenLineInfo {
                    line: 0,
                    column: 0,
                    file: "<builtin>".to_string(),
                },
            ),
            HezenValue::NativeFunction(Rc::new(HezenNativeFunction::new(
                Token::new(
                    TokenType::Builtin,
                    "println".to_string(),
                    HezenLineInfo {
                        line: 0,
                        column: 0,
                        file: "<builtin>".to_string(),
                    },
                ),
                1,
                |args| {
                    println!("{}", args[0]);
                    Ok(HezenValue::Nil)
                },
            ))),
            false,
        );

        globals.define(
            Token::new(
                TokenType::Builtin,
                "mod".to_string(),
                HezenLineInfo {
                    line: 0,
                    column: 0,
                    file: "<builtin>".to_string(),
                },
            ),
            HezenValue::NativeFunction(Rc::new(HezenNativeFunction::new(
                Token::new(
                    TokenType::Builtin,
                    "print".to_string(),
                    HezenLineInfo {
                        line: 0,
                        column: 0,
                        file: "<builtin>".to_string(),
                    },
                ),
                2,
                |args| {
                    if let (HezenValue::Number(left), HezenValue::Number(right)) =
                        (args[0].clone(), args[1].clone())
                    {
                        Ok(HezenValue::Number(left % right))
                    } else {
                        Err(HezenError::runtime(
                            "<builtin>".to_string(),
                            0,
                            0,
                            format!(
                                "Operands must be two numbers, not '{}' and '{}'",
                                args[0].type_name(),
                                args[1].type_name()
                            ),
                        ))
                    }
                },
            ))),
            false,
        );

        globals.define(
            Token::new(
                TokenType::Builtin,
                "show".to_string(),
                HezenLineInfo {
                    line: 0,
                    column: 0,
                    file: "<builtin>".to_string(),
                },
            ),
            HezenValue::NativeFunction(Rc::new(HezenNativeFunction::new(
                Token::new(
                    TokenType::Builtin,
                    "show".to_string(),
                    HezenLineInfo {
                        line: 0,
                        column: 0,
                        file: "<builtin>".to_string(),
                    },
                ),
                1,
                |args| {
                    Ok(match args[0] {
                        HezenValue::Nil => HezenValue::String("nil".to_string()),
                        _ => HezenValue::String(args[0].to_string()),
                    })
                },
            ))),
            false,
        );

        Self {
            globals: globals.clone(),
            environment: globals,
            locals: HashMap::default(),
        }
    }

    pub fn resolve(&mut self, token: &Token, depth: usize) {
        self.locals.insert(token.clone(), depth);
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), HezenError> {
        for statement in statements {
            let result = self.execute(statement);

            if let Err(HezenInterruption::Error(error)) = result {
                return Err(error);
            }

            if let Err(HezenInterruption::Control(_)) = result {
                panic!("Control flow should not be returned from the top level");
            }
        }

        Ok(())
    }

    pub(crate) fn execute(&mut self, stmt: &Stmt) -> Result<HezenValue, HezenInterruption> {
        match stmt {
            Stmt::Block(stmts) => self.execute_block(
                stmts.iter().collect(),
                HezenEnvironmentHandle::new(Some(self.environment.clone())),
            ),
            Stmt::Class(name, superclass, methods) => {
                let superclass = if let Some(superclass) = superclass {
                    match self
                        .evaluate(superclass)
                        .map_err(HezenInterruption::Error)?
                    {
                        HezenValue::Class(superclass) => Some(superclass),
                        x => {
                            return Err(HezenInterruption::Error(HezenError::runtime(
                                name.position.file.clone(),
                                name.position.line,
                                name.position.column,
                                format!("Superclass must be a class, not '{}'", x.type_name()),
                            )))
                        }
                    }
                } else {
                    None
                };

                self.environment.define(name.clone(), HezenValue::Nil, true);

                if superclass.is_some() {
                    self.environment = HezenEnvironmentHandle::new(Some(self.environment.clone()));
                    self.environment.define(
                        Token::new(
                            TokenType::Builtin,
                            "super".to_string(),
                            HezenLineInfo {
                                line: 0,
                                column: 0,
                                file: "<builtin>".to_string(),
                            },
                        ),
                        HezenValue::Class(superclass.as_ref().unwrap().clone()),
                        false,
                    );
                }

                let methods = methods
                    .iter()
                    .map(|method| {
                        if let Stmt::Function(name, params, body) = method {
                            let function = Rc::new(HezenFunction::new(
                                name.clone(),
                                params.clone(),
                                *body.clone(),
                                self.environment.clone(),
                                name.lexeme == "init",
                            ));

                            (name.lexeme.clone(), function)
                        } else {
                            unreachable!()
                        }
                    })
                    .collect::<HashMap<_, _>>();

                if superclass.is_some() {
                    self.environment = self.environment.clone().enclosing().unwrap();
                }

                let class = HezenValue::Class(Rc::new(HezenClass::new(
                    name.lexeme.clone(),
                    superclass,
                    methods,
                )));

                self.environment
                    .assign(&name.clone(), class.clone())
                    .map_err(HezenInterruption::Error)?;

                Ok(class)
            }
            Stmt::Expression(expr) => self.evaluate(expr).map_err(HezenInterruption::Error),
            Stmt::Function(name, parameters, body) => {
                let function = HezenValue::Function(Rc::new(HezenFunction::new(
                    name.clone(),
                    parameters.clone(),
                    *body.clone(),
                    self.environment.clone(),
                    false,
                )));

                self.environment.define(name.clone(), function, false);

                Ok(HezenValue::Nil)
            }
            Stmt::If(condition, then_block, else_block) => {
                if self
                    .evaluate(condition)
                    .map_err(HezenInterruption::Error)?
                    .is_truthy()
                {
                    self.execute(then_block)
                } else if let Some(else_block) = else_block {
                    self.execute(else_block)
                } else {
                    Ok(HezenValue::Nil)
                }
            }
            Stmt::Var(name, initializer) => {
                let value = if let Some(initializer) = initializer {
                    self.evaluate(initializer)
                        .map_err(HezenInterruption::Error)?
                } else {
                    HezenValue::Nil
                };

                self.environment.define(name.clone(), value, false);

                Ok(HezenValue::Nil)
            }
            Stmt::VarMut(name, initializer) => {
                let value = if let Some(initializer) = initializer {
                    self.evaluate(initializer)
                        .map_err(HezenInterruption::Error)?
                } else {
                    HezenValue::Nil
                };

                self.environment.define(name.clone(), value, true);

                Ok(HezenValue::Nil)
            }
            Stmt::While(condition, body) => {
                while self
                    .evaluate(condition)
                    .map_err(HezenInterruption::Error)?
                    .is_truthy()
                {
                    let result = self.execute(body);

                    if let Err(HezenInterruption::Control(HezenControl::Break)) = result {
                        break;
                    }

                    if let Err(HezenInterruption::Control(HezenControl::Continue)) = result {
                        continue;
                    }

                    if let Err(HezenInterruption::Error(_)) = result {
                        return result;
                    }
                }

                Ok(HezenValue::Nil)
            }
            Stmt::Return(_, expr) => {
                if let Some(expr) = expr {
                    let value = self.evaluate(expr).map_err(HezenInterruption::Error)?;
                    Err(HezenInterruption::Control(HezenControl::Return(value)))
                } else {
                    Err(HezenInterruption::Control(HezenControl::Return(
                        HezenValue::Nil,
                    )))
                }
            }
            Stmt::Break => Err(HezenInterruption::Control(HezenControl::Break)),
            Stmt::Continue => Err(HezenInterruption::Control(HezenControl::Continue)),
        }
    }

    pub(crate) fn evaluate(&mut self, expr: &Expr) -> Result<HezenValue, HezenError> {
        match expr {
            Expr::Assign(name, value) => {
                if self.locals.contains_key(name) {
                    if self
                        .environment
                        .mutable_at(*self.locals.get(name).unwrap(), name)
                    {
                        let value = self.evaluate(value)?;
                        self.environment
                            .assign_at(*self.locals.get(name).unwrap(), name, value)?;
                        Ok(HezenValue::Nil)
                    } else {
                        Err(HezenError::runtime(
                            name.position.file.clone(),
                            name.position.line,
                            name.position.column,
                            format!("Cannot assign to immutable variable '{}'", name.lexeme),
                        ))
                    }
                } else if self.globals.mutable(name) {
                    let value = self.evaluate(value)?;
                    self.globals.assign(name, value)?;
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
                            Ok(HezenValue::String(format!("{left}{right}")))
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
            Expr::Get(expr, token) => {
                let value = self.evaluate(expr)?;

                match value {
                    HezenValue::Instance(instance) => {
                        if let Some(value) = instance.get(&token.lexeme) {
                            Ok(value)
                        } else {
                            Err(HezenError::runtime(
                                token.position.file.clone(),
                                token.position.line,
                                token.position.column,
                                format!("Undefined property '{}'", token.lexeme),
                            ))
                        }
                    }
                    _ => Err(HezenError::runtime(
                        token.position.file.clone(),
                        token.position.line,
                        token.position.column,
                        format!(
                            "Only instances have properties, '{}' does not",
                            value.type_name()
                        ),
                    )),
                }
            }
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(l) => Ok(l.into()),
            Expr::Logical(left, op, right) => {
                let left = self.evaluate(left)?;

                match op.ty {
                    TokenType::Or => {
                        if left.is_truthy() {
                            Ok(left)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    TokenType::And => {
                        if !left.is_truthy() {
                            Ok(left)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    _ => Err(HezenError::runtime(
                        op.position.file.clone(),
                        op.position.line,
                        op.position.column,
                        format!("Invalid logical operator '{}'. Don't know how you did it, but that's a parser bug", op.lexeme),
                    )),
                }
            }
            Expr::Self_(token) => self.get(token, expr),
            Expr::Super(s, accessor) => {
                let distance = self.locals.get(s).unwrap();

                let superclass = match self.environment.get_at(*distance, s) {
                    Ok(HezenValue::Class(class)) => class,
                    _ => {
                        return Err(HezenError::runtime(
                            s.position.file.clone(),
                            s.position.line,
                            s.position.column,
                            "Can only access superclass from a subclass".to_string(),
                        ))
                    }
                };

                let object = match self.environment.get_at(
                    distance - 1,
                    &Token::new(TokenType::Self_, "self".into(), s.position.clone()),
                )? {
                    HezenValue::Instance(instance) => instance,
                    _ => {
                        return Err(HezenError::runtime(
                            s.position.file.clone(),
                            s.position.line,
                            s.position.column,
                            "Can only access superclass from a subclass".to_string(),
                        ))
                    }
                };

                let method = superclass.find_method(&accessor.lexeme);

                match method {
                    Some(method) => Ok(HezenValue::Function(Rc::new(method.bind(object)))),
                    None => Err(HezenError::runtime(
                        accessor.position.file.clone(),
                        accessor.position.line,
                        accessor.position.column,
                        format!("Undefined property '{}'", accessor.lexeme),
                    )),
                }
            }
            Expr::Set(obj, name, value) => {
                let obj = self.evaluate(obj)?;

                match obj {
                    HezenValue::Instance(instance) => {
                        let value = self.evaluate(value)?;

                        instance.set(name.lexeme.clone(), value.clone());

                        Ok(value)
                    }
                    _ => Err(HezenError::runtime(
                        name.position.file.clone(),
                        name.position.line,
                        name.position.column,
                        format!("Only instances have fields, '{}' does not", obj.type_name()),
                    )),
                }
            }
            Expr::Unary(op, right) => {
                let right = self.evaluate(right)?;

                match op.ty {
                    TokenType::Bang => Ok(HezenValue::Bool(!right.is_truthy())),
                    TokenType::Minus => match right {
                        HezenValue::Number(n) => Ok(HezenValue::Number(-n)),
                        _ => Err(HezenError::runtime(
                            op.position.file.clone(),
                            op.position.line,
                            op.position.column,
                            format!("Operand must be a number, not '{}'", right.type_name()),
                        )),
                    },
                    _ => Err(HezenError::runtime(
                        op.position.file.clone(),
                        op.position.line,
                        op.position.column,
                        format!("Invalid unary operator '{}'. Don't know how you did it, but that's a parser bug", op.lexeme),
                    )),
                }
            }
            Expr::Variable(name) => self.get(name, expr),
        }
    }

    pub(crate) fn execute_block(
        &mut self,
        stmts: Vec<&Stmt>,
        new_env: HezenEnvironmentHandle,
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

    fn get(&self, name: &Token, _expr: &Expr) -> Result<HezenValue, HezenError> {
        if let Some(distance) = self.locals.get(name) {
            self.environment.get_at(*distance, name)
        } else {
            self.globals.get(name)
        }
    }
}
