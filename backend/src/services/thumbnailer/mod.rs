use crate::services::{Locatable, Locator};
use crate::types::*;
use image::{imageops, io::Reader, DynamicImage};
use log::{debug, error};
use std::io::Cursor;
use std::panic;

pub struct ThumbnailerService;

impl ThumbnailerService {
    pub fn resize(&self, data: &[u8], size: u32) -> Result<Vec<u8>> {
        debug!("Reading an image to resize it to {} px.", size);

        let img = self.decode(data)?;

        if img.width() <= size && img.height() <= size {
            debug!("Image too small, no need to downsize.");
            return Ok(data.to_vec());
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

    pub fn validate(&self, data: &[u8]) -> Result<()> {
        self.decode(data)?;
        Ok(())
    }

    pub fn decode(&self, data: &[u8]) -> Result<DynamicImage> {
        debug!("Decoding image of {} bytes.", data.len());

        let reader = Reader::new(Cursor::new(data));

        let format = match reader.with_guessed_format() {
            Ok(value) => value,
            Err(e) => {
                error!("Error guessing image format: {:?}", e);
                return Err(Error::BadImage);
            }
        };

        let v1 = match panic::catch_unwind(|| format.decode()) {
            Ok(value) => value,

            Err(e) => {
                error!("Panic during decoding an image: {:?}.", e);
                return Err(Error::BadImage);
            }
        };

        let img = match v1 {
            Ok(value) => value,

            Err(e) => {
                error!("Error decoding image: {:?}", e);
                return Err(Error::BadImage);
            }
        };

        debug!("Image decoded, size is {}x{}", img.width(), img.height());

        Ok(img)
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

impl Locatable for ThumbnailerService {
    fn create(_locator: &Locator) -> Result<Self> {
        Ok(Self {})
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

        ThumbnailerService {}
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

    #[test]
    fn test_broken_image() {
        let thumbnailer = setup();

        let data = include_bytes!("test/broken.jpg");
        let resized = thumbnailer.resize(&data.to_vec(), 100);

        assert!(resized.is_err());
    }
}
