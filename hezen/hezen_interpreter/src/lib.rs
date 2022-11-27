use hezen_core::error::HezenErrorList;

mod ast;
mod lexer;
mod parser;
mod token;

pub fn run(filename: String, code: String) -> Result<(), HezenErrorList> {
    let mut errors = HezenErrorList::default();

    println!("Running file {}", filename);
    println!("Code:\n{}", code);

    let lexer = lexer::Lexer::new(filename, code, &mut errors);

    let tokens = lexer.get_tokens();

    if !errors.is_empty() {
        return Err(errors);
    }

    println!("Tokens: {}", tokens);

    let parser = parser::Parser::new(tokens, &mut errors);

    let ast = parser.parse();

    if !errors.is_empty() {
        return Err(errors);
    }

    for node in ast.iter() {
        println!("{}", node);
    }

    Ok(())
}
