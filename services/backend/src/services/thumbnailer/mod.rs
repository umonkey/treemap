use crate::services::{Context, Injectable};
use crate::types::*;
use image::{imageops, io::Reader, DynamicImage};
use log::{debug, error, info};
use std::io::Cursor;
use std::panic;

pub struct ThumbnailerService;

impl ThumbnailerService {
    /// Resize an image and return the JPEG bytes plus its dimensions.
    pub fn variant(&self, data: &[u8], max_size: u32) -> Result<(Vec<u8>, u32, u32)> {
        debug!("Reading an image to build a {} px variant.", max_size);

        let img = self.decode(data)?;

        info!(
            "Converting an {}x{} image to a {} px JPEG.",
            img.width(),
            img.height(),
            max_size
        );

        let rotated = self.autorotate(img, data)?;

        let resized = if rotated.width() <= max_size && rotated.height() <= max_size {
            rotated
        } else {
            rotated.thumbnail(max_size, max_size)
        };

        let width = resized.width();
        let height = resized.height();

        let mut buf: Vec<u8> = Vec::new();

        resized
            .into_rgb8()
            .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Jpeg)
            .map_err(|e| {
                error!("Error writing image: {e:?}");
                Error::ImageResize
            })?;

        debug!("Variant conversion successful, {}x{}.", width, height);

        Ok((buf, width, height))
    }

    pub fn decode(&self, data: &[u8]) -> Result<DynamicImage> {
        debug!("Decoding image of {} bytes.", data.len());

        let reader = Reader::new(Cursor::new(data));

        let format = match reader.with_guessed_format() {
            Ok(value) => value,
            Err(e) => {
                error!("Error guessing image format: {e:?}");
                return Err(Error::BadImage);
            }
        };

        let v1 = match panic::catch_unwind(|| format.decode()) {
            Ok(value) => value,

            Err(e) => {
                error!("Panic during decoding an image: {e:?}.");
                return Err(Error::BadImage);
            }
        };

        let img = match v1 {
            Ok(value) => value,

            Err(e) => {
                error!("Error decoding image: {e:?}");
                return Err(Error::BadImage);
            }
        };

        debug!("Image decoded, size is {}x{}", img.width(), img.height());

        Ok(img)
    }

    fn autorotate(&self, img: DynamicImage, raw_image: &[u8]) -> Result<DynamicImage> {
        let rotated = match self.get_orientation(raw_image)? {
            Some(90) => imageops::rotate90(&img).into(),
            Some(180) => imageops::rotate180(&img).into(),
            Some(270) => imageops::rotate270(&img).into(),
            _ => img,
        };

        Ok(rotated)
    }

    fn get_orientation(&self, data: &[u8]) -> Result<Option<u32>> {
        let reader = exif::Reader::new();
        let mut cursor = Cursor::new(data);

        let exif_data = match reader.read_from_container(&mut cursor) {
            Ok(value) => value,

            Err(e) => {
                debug!("Error reading EXIF data: {e}.");
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

impl Injectable for ThumbnailerService {
    fn inject(_ctx: &dyn Context) -> Result<Self> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use image::DynamicImage;
    use log::debug;

    fn setup() -> ThumbnailerService {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        ThumbnailerService {}
    }

    fn to_jpeg(thumbnailer: &ThumbnailerService, data: &[u8], max_size: u32) -> Result<Vec<u8>> {
        thumbnailer
            .variant(data, max_size)
            .map(|(bytes, _width, _height)| bytes)
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
        let resized = to_jpeg(&thumbnailer, data.as_ref(), 100).expect("Error resizing image.");

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
        let resized = to_jpeg(&thumbnailer, data.as_ref(), 2000).expect("Error resizing image.");

        let image = read_image(resized);
        assert_eq!(image.width(), 1023);
        assert_eq!(image.height(), 680);
    }

    #[test]
    fn test_autorotate() {
        let thumbnailer = setup();

        let data = include_bytes!("test/rotated.jpg");
        let resized = to_jpeg(&thumbnailer, data.as_ref(), 100).expect("Error resizing image.");

        let image = read_image(resized);

        assert_eq!(image.width(), 45);
        assert_eq!(image.height(), 100);
    }

    #[test]
    fn test_broken_image() {
        let thumbnailer = setup();

        let data = include_bytes!("test/broken.jpg");
        let resized = to_jpeg(&thumbnailer, data.as_ref(), 100);

        assert!(resized.is_err());
    }

    /**
     * Test converting a regular image to JPEG.
     *
     * The image is 1023x680, so converted to fit in a 2000x2000 square
     * the size should remain the same, and the result should still decode.
     */
    #[test]
    fn to_jpeg_large() {
        let thumbnailer = setup();

        let data = include_bytes!("test/tree.jpg");
        let jpeg = to_jpeg(&thumbnailer, data.as_ref(), 2000).expect("Error converting image.");

        assert_eq!(
            image::guess_format(&jpeg).expect("Error guessing format."),
            image::ImageFormat::Jpeg
        );

        let image = read_image(jpeg);
        assert_eq!(image.width(), 1023);
        assert_eq!(image.height(), 680);
    }

    /**
     * Test that images larger than the limit are downscaled.
     *
     * The image is 1023x680, so converted to fit in a 100x100 square
     * it should be 100x66 pixels, like the resize tests.
     */
    #[test]
    fn to_jpeg_downscales_large() {
        let thumbnailer = setup();

        let data = include_bytes!("test/tree.jpg");
        let jpeg = to_jpeg(&thumbnailer, data.as_ref(), 100).expect("Error converting image.");

        let image = read_image(jpeg);
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 66);
    }

    /**
     * Test that a PNG image comes back as JPEG, even when it is small
     * enough to need no resizing.
     */
    #[test]
    fn to_jpeg_converts_small_png() {
        let thumbnailer = setup();

        let mut png: Vec<u8> = Vec::new();

        DynamicImage::new_rgb8(10, 10)
            .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
            .expect("Error writing PNG.");

        let jpeg = to_jpeg(&thumbnailer, png.as_ref(), 2000).expect("Error converting image.");

        assert_eq!(
            image::guess_format(&jpeg).expect("Error guessing format."),
            image::ImageFormat::Jpeg
        );

        let image = read_image(jpeg);
        assert_eq!(image.width(), 10);
        assert_eq!(image.height(), 10);
    }

    #[test]
    fn variant_returns_dimensions() {
        let thumbnailer = setup();

        let data = include_bytes!("test/tree.jpg");
        let (jpeg, width, height) = thumbnailer
            .variant(data.as_ref(), 100)
            .expect("Error building variant.");

        assert_eq!(width, 100);
        assert_eq!(height, 66);

        let image = read_image(jpeg);
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 66);
    }
}
