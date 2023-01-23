use std::collections::HashMap;

use hezen_core::error::{HezenError, HezenErrorList};

use crate::{
    ast::{Expr, Stmt},
    class::ClassType,
    function::FunctionType,
    interpreter::Interpreter,
    token::Token,
};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
    errors: &'a mut HezenErrorList,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter, errors: &'a mut HezenErrorList) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
            errors,
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) {
        self.begin_scope();
        self.internal_resolve(statements);
        self.end_scope();
    }

    fn internal_resolve(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();

        scope.insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();

        scope.insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&mut self, name: &Token, expr: &Expr) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name, i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, function: &Stmt, function_type: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = function_type;

        self.begin_scope();

        if let Stmt::Function(name, params, body) = function {
            for param in params {
                self.declare(param);
                self.define(param);
            }

            if let Stmt::Block(statements) = &**body {
                self.internal_resolve(statements);
            } else {
                self.errors.add(HezenError::runtime(
                    name.position.file.clone(),
                    name.position.line,
                    name.position.column,
                    "Expected block statement in function body".into(),
                ));
            }
        } else {
            unreachable!()
        }

        self.end_scope();

        self.current_function = enclosing_function;
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(statements) => {
                self.begin_scope();
                self.internal_resolve(statements);
                self.end_scope();
            }
            Stmt::Class(name, superclass, methods) => {
                let enclosing_class = self.current_class;
                self.current_class = ClassType::Class;

                self.declare(name);
                self.define(name);

                if let Some(superclass) = superclass {
                    if let Expr::Variable(superclass) = superclass {
                        if superclass.lexeme == name.lexeme {
                            self.error(superclass.clone(), "A class cannot inherit from itself.");
                        }
                    }

                    self.current_class = ClassType::Subclass;

                    self.resolve_expr(superclass);

                    self.begin_scope();
                    self.scopes
                        .last_mut()
                        .unwrap()
                        .insert("super".to_string(), true);
                }

                self.begin_scope();
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert("self".to_string(), true);

                for method in methods {
                    if let Stmt::Function(name, _, _) = method {
                        let declaration = if name.lexeme == "init" {
                            FunctionType::Initializer
                        } else {
                            FunctionType::Method
                        };

                        self.resolve_function(method, declaration);
                    } else {
                        unreachable!()
                    }
                }

                self.end_scope();

                if superclass.is_some() {
                    self.end_scope();
                }

                self.current_class = enclosing_class;
            }
            Stmt::Expression(expr) => self.resolve_expr(expr),
            Stmt::Function(name, _, _) => {
                self.declare(name);
                self.define(name);

                self.resolve_function(stmt, FunctionType::Function);
            }
            Stmt::If(condition, then_branch, else_branch) => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);

                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(else_branch);
                }
            }
            Stmt::Return(keyword, value) => {
                if self.current_function == FunctionType::None {
                    self.error(keyword.clone(), "Cannot return from top-level code.");
                }

                if let Some(value) = value {
                    if self.current_function == FunctionType::Initializer {
                        self.error(
                            keyword.clone(),
                            "Cannot return a value from an initializer.",
                        );
                    }

                    self.resolve_expr(value);
                }
            }
            Stmt::Var(name, initializer) | Stmt::VarMut(name, initializer) => {
                self.declare(name);
                if let Some(initializer) = initializer {
                    self.resolve_expr(initializer);
                }
                self.define(name);
            }
            Stmt::While(condition, body) => {
                self.resolve_expr(condition);
                self.resolve_stmt(body);
            }
            Stmt::Break => {}
            Stmt::Continue => {}
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Assign(name, value) => {
                self.resolve_expr(value);
                self.resolve_local(name, expr);
            }
            Expr::Binary(left, _, right) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Call(callee, _, arguments) => {
                self.resolve_expr(callee);

                for argument in arguments {
                    self.resolve_expr(argument);
                }
            }
            Expr::Get(object, _) => self.resolve_expr(object),
            Expr::Grouping(expr) => self.resolve_expr(expr),
            Expr::Literal(_) => {}
            Expr::Logical(left, _, right) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Set(object, _, value) => {
                self.resolve_expr(object);
                self.resolve_expr(value);
            }
            Expr::Super(keyword, _) => {
                if self.current_class == ClassType::None {
                    self.error(keyword.clone(), "Cannot use 'super' outside of a class.");
                } else if self.current_class != ClassType::Subclass {
                    self.error(
                        keyword.clone(),
                        "Cannot use 'super' in a class with no superclass.",
                    );
                }

                self.resolve_local(keyword, expr);
            }
            Expr::Self_(keyword) => {
                if self.current_class == ClassType::None {
                    self.error(keyword.clone(), "Cannot use 'self' outside of a class.");
                    return;
                }

                self.resolve_local(keyword, expr);
            }
            Expr::Unary(_, right) => self.resolve_expr(right),
            Expr::Variable(name) => {
                if !self.scopes.is_empty()
                    && self.scopes.last().unwrap().get(&name.lexeme) == Some(&false)
                {
                    self.error(
                        name.clone(),
                        "Cannot read local variable in its own initializer.",
                    );
                }

                self.resolve_local(name, expr);
            }
        }
    }

    fn error(&mut self, token: Token, message: &str) {
        self.errors.add(HezenError::validation(
            token.position.file,
            token.position.line,
            token.position.column,
            message.into(),
        ));
    }
}
