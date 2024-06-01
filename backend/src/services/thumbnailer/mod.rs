/**
 * This module contains image resizing code.
 */
use image::{imageops, io::Reader, DynamicImage};
use log::{debug, error};
use std::io::Cursor;

use crate::types::{Error, Result};

pub struct ThumbnailerService;

impl ThumbnailerService {
    pub fn new() -> Self {
        Self
    }

    pub fn resize(&self, data: &Vec<u8>, size: u32) -> Result<Vec<u8>> {
        debug!("Reading an image to resize it to {} px.", size);

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

        debug!("Image read, size is {}x{}", img.width(), img.height());

        if img.width() <= size && img.height() <= size {
            return Ok(data.clone());
        }

        let rotated = self.autorotate(&img, data)?;
        let resized = rotated.resize(size, size, image::imageops::FilterType::Lanczos3);

        let mut buf: Vec<u8> = Vec::new();

        resized
            .to_rgb8()
            .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Jpeg)
            .map_err(|e| {
                error!("Error writing image: {:?}", e);
                Error::ImageResize
            })?;

        debug!("Resize successful.");

        Ok(buf)
    }

    fn autorotate(&self, img: &DynamicImage, raw_image: &[u8]) -> Result<DynamicImage> {
        let rotated = match self.get_orientation(raw_image)? {
            Some(90) => imageops::rotate90(img),
            Some(180) => imageops::rotate180(img),
            Some(270) => imageops::rotate270(img),
            _ => img.clone().into(),
        };

        Ok(rotated.into())
    }

    fn get_orientation(&self, data: &[u8]) -> Result<Option<u32>> {
        let reader = exif::Reader::new();
        let mut cursor = Cursor::new(data);

        let exif_data = match reader.read_from_container(&mut cursor) {
            Ok(value) => value,

            Err(e) => {
                debug!("Error reading EXIF data: {}.", e);
                return Ok(None);
            }
        };

        let exif_field = match exif_data.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
            Some(value) => value,
            None => return Ok(None),
        };

        match exif_field.value.get_uint(0) {
            Some(1) => Ok(None),
            Some(3) => Ok(Some(180)),
            Some(6) => Ok(Some(90)),
            Some(8) => Ok(Some(270)),

            _ => {
                debug!(
                    "Unknown EXIF orientation: {:?}",
                    exif_field.value.get_uint(0)
                );
                Ok(None)
            }
        }
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

        ThumbnailerService::new()
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

    #[test]
    fn test_autorotate() {
        let thumbnailer = setup();

        let data = include_bytes!("test/rotated.jpg");
        let resized = thumbnailer
            .resize(&data.to_vec(), 100)
            .expect("Error resizing image.");

        let image = read_image(resized);

        assert_eq!(image.width(), 45);
        assert_eq!(image.height(), 100);
    }
}
