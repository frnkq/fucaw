const MSG_HELP: &str = "No arguments were provided";

pub fn print_help() {
    print(MSG_HELP);
}

fn print(txt: &str) {
    println!("{}", &txt);
}
