mod command_line;
pub use std::env;
pub use crate::command_line::arguments;

pub fn run () {
    let arguments = arguments::parse_arguments(env::args().collect());
    println!("{:?}", arguments);
}



