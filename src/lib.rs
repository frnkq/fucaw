mod cli;
mod wallpaper;

use std::{env,process};
use cli::{arguments,commands,output}; 

use wallpaper::creator::create_wallpaper;
use wallpaper::manager::set_wallpaper;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        output::print_help();
        process::exit(output::EXIT_MISSING_ARGUMENTS);
    }

    let commands = commands::frequently_used();
    println!("{:?}", commands);

    let args = arguments::parse_arguments(args);
    let path = create_wallpaper(&args.source_image_path).unwrap();
    set_wallpaper(path).unwrap();
    process::exit(output::EXIT_OK);
}
