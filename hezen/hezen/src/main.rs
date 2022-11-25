use anyhow::Result;
use clap::Parser;
use std::{io::Read, path::PathBuf};

#[derive(Parser, Clone, Debug)]
#[command(version = "1.0", about = "The Hezen Language")]
struct Args {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Clone, Debug)]
enum SubCommand {
    #[command(name = "run")]
    Run { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcmd {
        SubCommand::Run { file } => run(file),
    }
}

fn run(file: Option<PathBuf>) -> Result<()> {
    if let Some(file) = file {
        let filename = file.to_str().unwrap().to_string();
        let code = std::fs::read_to_string(file)?;
        hezen_runtime::run(filename, code)?;
    } else {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        let mut code = String::new();
        stdin.read_to_string(&mut code)?;
        let result = hezen_runtime::run(String::from("<stdin>"), code);

        if let Err(err) = result {
            eprintln!("{}", err);
        }
    }
    Ok(())
}
