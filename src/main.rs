use clap::Parser;
use std::path::PathBuf;

use crate::runner::runner::Runner;

mod jq;
mod runner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    libraries: Vec<PathBuf>,

    #[arg(short, long)]
    module: String,
}

fn main() {
    let args = Arguments::parse();

    let runner = Runner::start();

    args.libraries
        .into_iter()
        .map(|path| path.canonicalize().expect("should have a valid path"))
        .for_each(|library| runner.add_library(library));

    runner
        .get_functions_for_module(&args.module)
        .into_iter()
        .filter(|function| function.starts_with("should_"))
        .map(|function| runner.execute_test(&args.module, &function))
        .for_each(|test_result| println!("{}", test_result));
}
