mod cli;

use cli::arguments;
use cli::output;
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        output::print_help();
    }
    arguments::parse_arguments(args);
}
