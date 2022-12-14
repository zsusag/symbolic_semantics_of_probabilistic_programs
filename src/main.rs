use anyhow::{Context, Result};
use clap::Parser;

use pest::Parser as EParser;
use std::fs;

use crate::{
    executor::Executor,
    parser::{parse_file, ExprParser},
    semantics::check_valid_program,
};

mod executor;
mod executor_state;
mod expr;
mod parser;
mod path;
mod probability;
mod psi_parser;
mod semantics;
mod smt;
mod syntax;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to probabilistic programs
    p: std::path::PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // Need to check whether file exists
    let f = fs::read_to_string(args.p.as_path())
        .with_context(|| format!("Failed to read program from {}", args.p.as_path().display()))?;

    let fn_defs = parse_file(
        ExprParser::parse(parser::Rule::file, &f)
            .context("Failed to parse function definitions")?,
    );

    check_valid_program(&fn_defs)?;

    let executor = Executor::new(fn_defs);
    let paths = executor.run()?;

    println!("Number of Paths: {}", paths.len());

    for (i, path) in paths.iter().enumerate() {
        println!("Path {}:\n\t{}", i + 1, path);
    }
    Ok(())
}
