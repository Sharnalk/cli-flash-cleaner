mod args;
mod parser;

use args::Args;
use clap::Parser;

use crate::parser::cli_parser;

fn main() {
    let parse = Args::parse();
    cli_parser(&parse);
}
