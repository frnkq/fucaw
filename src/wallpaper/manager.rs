use std::io::Result;
use std::process::Command;
use std::process::ExitStatus;

pub fn create_image(image_path: &str) {
    let mut imgbuf = image::ImageBuffer::new(800, 800);

    imgbuf.save("/tmp/fucaw.png");
}

pub fn set_wallpaper(img_path: &str) -> Result<ExitStatus> {
    return Command::new("feh").arg("--bg-fill").arg(img_path).status();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls_wallpaper_setter_command() {
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

    #[test]
    fn creates_image(){
        let image_path: &str = "/tmp/fucaw.png";
        create_image(image_path);
        assert_eq!(true, std::path::Path::new(image_path).is_file());
    }
}
