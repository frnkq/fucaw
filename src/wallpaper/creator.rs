use image::{RgbImage, ImageError };

pub fn create_image(image_path: &str)->Result<(), ImageError> {
    let imgbuff: RgbImage = RgbImage::new(300, 300);
    match imgbuff.save(image_path){
        Ok(img) => return Ok(img),
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
