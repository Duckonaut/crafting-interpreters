use std::{error::Error, fmt::Display};

#[cfg(feature = "color")]
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct HezenLineInfo {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl HezenLineInfo {
    pub fn new(file: String, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }
}

#[derive(Debug)]
pub enum HezenError {
    Syntax(HezenLineInfo, String),
    Validation(HezenLineInfo, String),
    Runtime(HezenLineInfo, String),
}

impl Display for HezenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HezenError::Syntax(info, msg) => write!(
                f,
                "SyntaxError: {} at {}:{}:{}",
                msg, info.file, info.line, info.column
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
    pub fn syntax_error(file: String, line: usize, column: usize, msg: String) -> Self {
        Self::Syntax(HezenLineInfo { file, line, column }, msg)
    }

    pub fn validation(file: String, line: usize, column: usize, msg: String) -> Self {
        Self::Validation(HezenLineInfo { file, line, column }, msg)
    }

    pub fn runtime(file: String, line: usize, column: usize, msg: String) -> Self {
        Self::Runtime(HezenLineInfo { file, line, column }, msg)
    }

    #[cfg(feature = "color")]
    pub fn print_details<'a>(
        &self,
        f: impl std::fmt::Write,
        source: impl Into<&'a str>,
    ) -> std::fmt::Result {
        match self {
            HezenError::Syntax(info, msg) => {
                self.print_internal(f, source, info, "syntax error", msg)
            }
            HezenError::Validation(info, msg) => {
                self.print_internal(f, source, info, "validation error", msg)
            }
            HezenError::Runtime(info, msg) => {
                self.print_internal(f, source, info, "runtime error", msg)
            }
        }
    }

    #[cfg(feature = "color")]
    fn print_internal<'a>(
        &self,
        mut f: impl std::fmt::Write,
        source: impl Into<&'a str>,
        info: &HezenLineInfo,
        prefix: &str,
        msg: &str,
    ) -> std::fmt::Result {
        let source = source.into();
        let lines = source.lines().collect::<Vec<&str>>();
        let line_max_len = lines.len().to_string().len();

        writeln!(f, "{}: {}", prefix.red(), msg)?;
        writeln!(
            f,
            "{} {}:{}:{}",
            " -->".bright_blue(),
            info.file,
            info.line,
            info.column
        )?;
        let line = lines[info.line - 1];
        let line_num = info.line.to_string();
        let line_num = format!("{: >1$} |", line_num, line_max_len).bright_blue();

        let padding = format!("{}{}", " ".repeat(line_num.len() - 1), "|".bright_blue());

        writeln!(f, "{}", padding)?;
        writeln!(f, "{} {}", line_num, line)?;
        write!(f, "{}", padding)?;
        writeln!(f, "{}{}", " ".repeat(info.column), "^".bright_red())?;
        writeln!(f, "{}", padding)
    }
}

#[derive(Debug, Default)]
pub struct HezenErrorList {
    errors: Vec<HezenError>,
}

impl HezenErrorList {
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

    #[cfg(feature = "color")]
    pub fn print_details<'a>(
        &self,
        mut f: impl std::fmt::Write,
        source: impl Into<&'a str> + Clone,
    ) -> std::fmt::Result {
        for error in &self.errors {
            error.print_details(&mut f, source.clone())?;
        }
        Ok(())
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
