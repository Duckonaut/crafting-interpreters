use std::io::Write;

use hezen_core::{error::HezenErrorList, Verbosity};

mod ast;
mod class;
mod environment;
mod function;
mod instance;
mod interpreter;
mod lexer;
mod parser;
mod resolver;
mod token;

pub fn run(filename: String, code: String, verbosity: Verbosity) -> Result<(), HezenErrorList> {
    let mut pre_run_errors = HezenErrorList::default();

    let lexer = lexer::Lexer::new(filename, code, &mut pre_run_errors);

    let tokens = lexer.get_tokens();

    if !pre_run_errors.is_empty() {
        return Err(pre_run_errors);
    }

    if verbosity.lexer {
        println!("Tokens: {tokens}");
    }

    let parser = parser::Parser::new(tokens, &mut pre_run_errors);

    let ast = parser.parse();

    if !pre_run_errors.is_empty() {
        return Err(pre_run_errors);
    }

    if verbosity.intermediate {
        println!("AST:");
        for node in ast.iter() {
            println!("{node}");
        }
    }

    let mut interpreter = interpreter::Interpreter::new();

    let mut resolver = resolver::Resolver::new(&mut interpreter, &mut pre_run_errors);

    resolver.resolve(&ast);

    if verbosity.resolver {
        for (local, dist) in interpreter.locals.iter() {
            println!(
                "{}: {}, at {}:{}",
                local.lexeme, dist, local.position.line, local.position.column
            );
        }
    }

    if !pre_run_errors.is_empty() {
        return Err(pre_run_errors);
    }

    let result = interpreter.interpret(&ast);

    if let Err(error) = result {
        return Err(HezenErrorList::from(error));
    }

    Ok(())
}

pub fn shell() {
    println!("Hezen Interpreter");
    println!("Type 'exit' to exit the shell");
    println!("Type 'help' to get help");

    let mut interpreter = interpreter::Interpreter::new();

    loop {
        let mut input = String::new();

        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input == "help" {
            println!("Type 'exit' to exit the shell");
            println!("Type 'help' to get help");
            continue;
        }

        let mut pre_run_errors = HezenErrorList::default();

        let lexer = lexer::Lexer::new("shell".to_string(), input.to_string(), &mut pre_run_errors);

        let tokens = lexer.get_tokens();

        if !pre_run_errors.is_empty() {
            let mut buffer = String::new();
            pre_run_errors.print_details(&mut buffer, input).unwrap();
            eprintln!("{buffer}");
            continue;
        }

        let parser = parser::Parser::new(tokens, &mut pre_run_errors);

        let ast = parser.parse();

        if !pre_run_errors.is_empty() {
            let mut buffer = String::new();
            pre_run_errors.print_details(&mut buffer, input).unwrap();
            eprintln!("{buffer}");
            continue;
        }

        let mut resolver = resolver::Resolver::new(&mut interpreter, &mut pre_run_errors);

        resolver.resolve(&ast);

        if !pre_run_errors.is_empty() {
            let mut buffer = String::new();
            pre_run_errors.print_details(&mut buffer, input).unwrap();
            eprintln!("{buffer}");
            continue;
        }

        let result = interpreter.interpret(&ast);

        if let Err(error) = result {
            let mut buffer = String::new();
            error.print_details(&mut buffer, input).unwrap();
            eprintln!("{buffer}");
        }
    }
}
