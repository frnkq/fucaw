pub const EXIT_OK: i32 = 0;
pub const EXIT_MISSING_ARGUMENTS: i32 = 1;
pub const MSG_HELP: &str = "No arguments were provided";

pub fn print_help() {
    print(MSG_HELP);
}

fn print(txt: &str) {
    println!("{}", &txt);
}
