use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::runner::{find_test_modules, Runner};

mod jq;
mod runner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Libraries to import when executing tests
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    libraries: Vec<PathBuf>,

    /// Module to execute tests within
    #[arg(short, long)]
    module: Option<String>,
}

fn main() -> ExitCode {
    let args = Arguments::parse();

    let runner = Runner::start();

    let mut library_modules = vec![];

    args.libraries
        .into_iter()
        .map(|path| path.canonicalize().expect("should have a valid path"))
        .for_each(|library| {
            runner.add_library(library.clone());
            library_modules.extend(find_test_modules(library.to_path_buf()))
        });

    let mut pass = true;

    args.module
        .map(|module| vec![module])
        .unwrap_or(library_modules)
        .iter()
        .for_each(|module| {
            runner
                .get_functions_for_module(module)
                .into_iter()
                .filter(|function| function.starts_with("should_"))
                .map(|function| runner.execute_test(module, &function))
                .for_each(|result| {
                    pass &= result.pass;
                    println!("{}", result);
                });
        });

    if pass {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
