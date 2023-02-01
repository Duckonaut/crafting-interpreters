use anyhow::Result;
use clap::Parser;
use hezen_core::Verbosity;
use std::{io::Read, path::PathBuf};

#[derive(Parser, Clone, Debug)]
#[command(version = "1.0", about = "The Hezen Language")]
struct Args {
    #[arg(short, long, help = "The verbosity level")]
    verbosity: Option<u8>,
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

    let verbosity = get_verbosity(args.verbosity);

    match args.subcmd {
        SubCommand::Run { file } => run(file, verbosity)?,
        SubCommand::Shell => shell(),
    }

    Ok(())
}

fn get_verbosity(level: Option<u8>) -> Verbosity {
    let mut v = Verbosity {
        lexer: false,
        intermediate: false,
        resolver: false,
    };

    if let Some(level) = level {
        if level > 0 {
            v.intermediate = true;
        }
        if level > 1 {
            v.resolver = true;
        }
        if level > 2 {
            v.lexer = true;
        }
    }

    v
}

fn run(file: Option<PathBuf>, verbosity: Verbosity) -> Result<()> {
    if let Some(file) = file {
        let filename = file.to_str().unwrap().to_string();
        let code = std::fs::read_to_string(file)?;
        let result = hezen_runtime::run(filename, code.clone(), verbosity);

        if let Err(err) = result {
            let mut buffer = String::new();
            err.print_details(&mut buffer, &*code).unwrap();
            eprintln!("{buffer}");
        }
    } else {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        let mut code = String::new();
        stdin.read_to_string(&mut code)?;
        let result = hezen_runtime::run(String::from("<stdin>"), code.clone(), verbosity);

        if let Err(err) = result {
            let mut buffer = String::new();
            err.print_details(&mut buffer, &*code).unwrap();
            eprintln!("{buffer}");
        }
    }
    Ok(())
}

fn shell() {
    hezen_runtime::shell();
}
