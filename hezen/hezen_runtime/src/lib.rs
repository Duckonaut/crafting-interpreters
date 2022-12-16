use hezen_core::error::HezenErrorList;
use hezen_core::Verbosity;

#[cfg(all(feature = "interpreter", feature = "compiler"))]
compile_error!(
    "Only one of the features `interpreter` and `compiler` can be enabled at the same time."
);

#[cfg(feature = "interpreter")]
pub fn run(filename: String, code: String, verbosity: Verbosity) -> Result<(), HezenErrorList> {
    hezen_interpreter::run(filename, code, verbosity)
}

#[cfg(feature = "interpreter")]
pub fn shell() {
    hezen_interpreter::shell()
}

#[cfg(feature = "compiler")]
pub fn run(filename: String, code: String, verbosity: Verbosity) -> Result<(), HezenErrorList> {
    hezen_compiler::run(filename, code, verbosity)
}

#[cfg(feature = "compiler")]
pub fn shell() {
    unimplemented!()
}
