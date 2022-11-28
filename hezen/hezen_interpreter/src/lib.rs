use std::io::Write;

use hezen_core::error::HezenErrorList;

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

pub fn run(filename: String, code: String) -> Result<(), HezenErrorList> {
    let mut pre_run_errors = HezenErrorList::default();

    println!("Running file {}", filename);
    println!("Code:\n{}", code);

    let lexer = lexer::Lexer::new(filename, code, &mut pre_run_errors);

    let tokens = lexer.get_tokens();

    if !pre_run_errors.is_empty() {
        return Err(pre_run_errors);
    }

    println!("Tokens: {}", tokens);

    let parser = parser::Parser::new(tokens, &mut pre_run_errors);

    let ast = parser.parse();

    if !pre_run_errors.is_empty() {
        return Err(pre_run_errors);
    }

    for node in ast.iter() {
        println!("{}", node);
    }

    let mut interpreter = interpreter::Interpreter::new();

    let mut resolver = resolver::Resolver::new(&mut interpreter, &mut pre_run_errors);

    resolver.resolve(&ast);

    if !pre_run_errors.is_empty() {
        return Err(pre_run_errors);
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
            eprintln!("{}", buffer);
            continue;
        }

        let parser = parser::Parser::new(tokens, &mut pre_run_errors);

        let ast = parser.parse();

        if !pre_run_errors.is_empty() {
            let mut buffer = String::new();
            pre_run_errors.print_details(&mut buffer, input).unwrap();
            eprintln!("{}", buffer);
            continue;
        }

        let mut resolver = resolver::Resolver::new(&mut interpreter, &mut pre_run_errors);

        resolver.resolve(&ast);

        if !pre_run_errors.is_empty() {
            let mut buffer = String::new();
            pre_run_errors.print_details(&mut buffer, input).unwrap();
            eprintln!("{}", buffer);
            continue;
        }

        let result = interpreter.interpret(&ast);

        if let Err(error) = result {
            println!("{}", error);
        }
    }
}
