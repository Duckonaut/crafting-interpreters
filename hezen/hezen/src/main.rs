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
    #[command(name = "shell")]
    Shell,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcmd {
        SubCommand::Run { file } => run(file)?,
        SubCommand::Shell => shell(),
    }

    Ok(())
}

fn run(file: Option<PathBuf>) -> Result<()> {
    if let Some(file) = file {
        let filename = file.to_str().unwrap().to_string();
        let code = std::fs::read_to_string(file)?;
        let result = hezen_runtime::run(filename, code.clone());

        if let Err(err) = result {
            let mut buffer = String::new();
            err.print_details(&mut buffer, &*code).unwrap();
            eprintln!("{}", buffer);
        }
    } else {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        let mut code = String::new();
        stdin.read_to_string(&mut code)?;
        let result = hezen_runtime::run(String::from("<stdin>"), code.clone());

        if let Err(err) = result {
            let mut buffer = String::new();
            err.print_details(&mut buffer, &*code).unwrap();
            eprintln!("{}", buffer);
        }
    }
    Ok(())
}

fn shell() {
    hezen_runtime::shell();
}
