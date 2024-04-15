use image::io::Reader;
use log::error;
/**
 * This module contains image resizing code.
 */
use std::io::Cursor;

use crate::types::Error;
use crate::Result;

pub struct ThumbnailerService;

impl ThumbnailerService {
    pub fn init() -> Self {
        Self
    }

    pub fn resize(&self, data: &Vec<u8>, size: u32) -> Result<Vec<u8>> {
        let img = Reader::new(Cursor::new(data))
            .with_guessed_format()
            .map_err(|e| {
                error!("Error guessing image format: {:?}", e);
                Error::BadImage
            })?
            .decode()
            .map_err(|e| {
                error!("Error decoding image: {:?}", e);
                Error::BadImage
            })?;

        if img.width() <= size && img.height() <= size {
            return Ok(data.clone());
        }

        let img = img.resize(size, size, image::imageops::FilterType::Lanczos3);

        let mut buf: Vec<u8> = Vec::new();

        img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Jpeg)
            .map_err(|e| {
                error!("Error writing image: {:?}", e);
                Error::ImageResize
            })?;

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use image::DynamicImage;
    use log::debug;

    fn setup() -> ThumbnailerService {
        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        ThumbnailerService::init()
    }

    fn read_image(bytes: Vec<u8>) -> DynamicImage {
        Reader::new(Cursor::new(bytes))
            .with_guessed_format()
            .expect("Error guessing image format.")
            .decode()
            .expect("Error decoding image.")
    }

    /**
     * Test resizing an image that is larger than required.
     *
     * The image is 1023x680, so when resized to fit in a 100x100 square, it should
     * be 100x66 pixels.
     */
    #[test]
    fn resize_large() {
        let thumbnailer = setup();

        let data = include_bytes!("test/tree.jpg");
        let resized = thumbnailer
            .resize(&data.to_vec(), 100)
            .expect("Error resizing image.");

        let image = read_image(resized);
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 66);
    }

    /**
     * Test resizing an image that is larger than required.
     *
     * The image is 1023x680, so when resized to fit in a 2000x2000 square,
     * the size should remain the same.
     */
    #[test]
    fn resize_small() {
        let thumbnailer = setup();

        let data = include_bytes!("test/tree.jpg");
        let resized = thumbnailer
            .resize(&data.to_vec(), 2000)
            .expect("Error resizing image.");

        let image = read_image(resized);
        assert_eq!(image.width(), 1023);
        assert_eq!(image.height(), 680);
    }
}
