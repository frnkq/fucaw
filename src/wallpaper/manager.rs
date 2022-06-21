use std::io::Result;
use std::process::Command;
use std::process::ExitStatus;

pub fn set_wallpaper(img_path: &str) -> Result<ExitStatus> {
    //receive img (see return type of wallcreator
    //get it's path? if no path, persist it
    //
    return Command::new("feh").arg("--bg-fill").arg(img_path).status();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_wallpaper() {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 1 {
            let output = set_wallpaper("");
            assert_eq!(true, output.is_err())
        } else {
            let img_path: &str = &args[0];
            let output = set_wallpaper(img_path);
            assert_eq!(true, output.is_ok())
        }
    }
}
