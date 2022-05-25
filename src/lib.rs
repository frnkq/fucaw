mod cli;

use cli::arguments;
use cli::output;
use std::env;
use std::process;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        output::print_help();
        process::exit(output::EXIT_MISSING_ARGUMENTS);
    }
    arguments::parse_arguments(args);
    process::exit(output::EXIT_OK);
}
