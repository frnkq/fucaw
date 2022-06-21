mod cli;
mod wallpaper;

use cli::arguments;
use cli::commands;
use cli::output;
use std::env;
use std::process;
use wallpaper::creator as wallcreator;
use wallpaper::manager as wallmanager;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        output::print_help();
        process::exit(output::EXIT_MISSING_ARGUMENTS);
    }

    let commands = commands::frequently_used();
    println!("{:?}", commands);

    let args = arguments::parse_arguments(args);
    let image = wallcreator::create_image(&args.image_path);
    wallmanager::set_wallpaper(&args.image_path);

    process::exit(output::EXIT_OK);
}
