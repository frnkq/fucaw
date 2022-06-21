use image::{RgbImage, ImageError };

pub fn create_wallpaper(source_image_path: &str)->Result<&str, ImageError> {
    let new_path = "/tmp/fucaw.png"; //move to config
    let imgbuff: RgbImage = RgbImage::new(300, 300);
    match imgbuff.save(new_path){
        Ok(_img) => return Ok(new_path),
        Err(e) => return Err(e)
    };
}

#[cfg(tests)]
mod tests {
    #[test]
    fn creates_image(){
        let image_path: &str = "/tmp/fucaw.png";
        create_image(image_path).unwrap();
        assert_eq!(true, std::path::Path::new(image_path).is_file());
    
    }
}
