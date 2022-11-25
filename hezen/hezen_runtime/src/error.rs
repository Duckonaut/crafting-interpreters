use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum HezenError {
    Parser(HezenLineInfo, String),
    Validation(HezenLineInfo, String),
    Runtime(HezenLineInfo, String),
}

#[derive(Debug)]
pub struct HezenLineInfo {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Display for HezenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HezenError::Parser(info, msg) => write!(
                f,
                "Parser error in file {} at line {}:{}: {}",
                info.file, info.line, info.column, msg
            ),
            HezenError::Validation(info, msg) => write!(
                f,
                "Validation error in file {} at line {}:{}: {}",
                info.file, info.line, info.column, msg
            ),
            HezenError::Runtime(info, msg) => write!(
                f,
                "Runtime error in file {} at line {}:{}: {}",
                info.file, info.line, info.column, msg
            ),
        }
    }
}

impl Error for HezenError {}

impl HezenError {
    pub fn parser(file: String, line: usize, column: usize, msg: String) -> Self {
        Self::Parser(HezenLineInfo { file, line, column }, msg)
    }

    pub fn validation(file: String, line: usize, column: usize, msg: String) -> Self {
        Self::Validation(HezenLineInfo { file, line, column }, msg)
    }

    pub fn runtime(file: String, line: usize, column: usize, msg: String) -> Self {
        Self::Runtime(HezenLineInfo { file, line, column }, msg)
    }
}

#[derive(Debug)]
pub struct HezenErrorList {
    errors: Vec<HezenError>,
}

impl HezenErrorList {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn add(&mut self, error: HezenError) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, HezenError> {
        self.errors.iter()
    }
}

impl Display for HezenErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl Error for HezenErrorList {}
