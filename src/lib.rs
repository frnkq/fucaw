mod cli;
mod wallpaper;

use cli::arguments;
use cli::commands;
use cli::output;
use std::env;
use std::process;
use wallpaper::manager as wallmanager;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        output::print_help();
        process::exit(output::EXIT_MISSING_ARGUMENTS);
    }
    let args = arguments::parse_arguments(args);
    let bash_history = commands::read_terminal_history();

    wallmanager::set_wallpaper(&args.image_path);
    process::exit(output::EXIT_OK);
}
