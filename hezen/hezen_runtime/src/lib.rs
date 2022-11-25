use error::HezenError;

use crate::error::HezenErrorList;

mod error;
mod lexer;

pub fn run(filename: String, code: String) -> Result<(), HezenError> {
    let mut errors = HezenErrorList::new();

    println!("Running file {}", filename);
    println!("Code:\n{}", code);

    Ok(())
}
