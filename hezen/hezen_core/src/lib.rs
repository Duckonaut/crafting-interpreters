pub mod error;

pub struct Verbosity {
    pub lexer: bool,
    pub intermediate: bool,
    pub resolver: bool,
}
