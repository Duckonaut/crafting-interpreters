use hezen_core::error::HezenErrorList;

#[cfg(all(feature = "interpreter", feature = "compiler"))]
compile_error!("Only one of the features `interpreter` and `compiler` can be enabled at the same time.");

#[cfg(feature = "interpreter")]
pub fn run(filename: String, code: String) -> Result<(), HezenErrorList> {
    hezen_interpreter::run(filename, code)
}

#[cfg(feature = "compiler")]
pub fn run(filename: String, code: String) -> Result<(), HezenErrorList> {
    hezen_compiler::run(filename, code)
}
