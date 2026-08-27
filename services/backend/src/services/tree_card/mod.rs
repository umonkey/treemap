use crate::domain::tree::Tree;
use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::infra::storage::FileBucket;
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use ab_glyph::{FontArc, PxScale};
use image::{imageops, io::Reader as ImageReader, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use log::{debug, error, info, warn};
use std::io::Cursor;
use std::sync::Arc;

const REGULAR_FONT_BYTES: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf");
const ITALIC_FONT_BYTES: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Italic.ttf");

pub struct TreeCardService {
    config: Arc<Config>,
    secrets: Arc<Secrets>,
    storage: Arc<FileBucket>,
    http: reqwest::Client,
    regular_font: FontArc,
    italic_font: FontArc,
}

impl TreeCardService {
    pub async fn get_card(&self, tree: &Tree) -> Result<Vec<u8>> {
        debug!(
            "Handling card generation for tree ID: {}, thumbnail_id: {:?}, images_updated_at: {}",
            tree.id, tree.thumbnail_id, tree.images_updated_at
        );

        let mut canvas = RgbImage::from_pixel(1200, 630, Rgb([244, 246, 244]));

        let left_block_rect = Rect::at(20, 20).of_size(570, 492);
        draw_filled_rect_mut(&mut canvas, left_block_rect, Rgb([255, 255, 255]));

        let photo_drawn = if let Some(thumb_id) = tree.thumbnail_id {
            debug!("Attempting to read thumbnail file ID: {:?}", thumb_id);
            match self.storage.read_file(thumb_id).await {
                Ok(bytes) => match self.decode_and_crop_image(&bytes, 570, 492) {
                    Ok(img) => {
                        debug!("Thumbnail read successfully ({} bytes), decoding and cropping image to 570x492", bytes.len());
                        imageops::overlay(&mut canvas, &img, 20, 20);
                        true
                    }
                    Err(e) => {
                        warn!("Failed to decode/crop thumbnail image: {:?}. Falling back to placeholder graphic.", e);
                        false
                    }
                },
                Err(e) => {
                    warn!("Failed to read thumbnail file ID {:?}: {:?}. Falling back to placeholder graphic.", thumb_id, e);
                    false
                }
            }
        } else {
            debug!(
                "Tree ID {} has no thumbnail_id, falling back to placeholder graphic.",
                tree.id
            );
            false
        };

        if !photo_drawn {
            let placeholder_rect = Rect::at(20, 20).of_size(570, 492);
            draw_filled_rect_mut(&mut canvas, placeholder_rect, Rgb([226, 232, 240]));

            let ph_text = "No Photo Available";
            let ph_scale = PxScale::from(24.0);
            let (ph_w, ph_h) = text_size(ph_scale, &self.regular_font, ph_text);
            let ph_x = 20 + (570 - ph_w as i32) / 2;
            let ph_y = 20 + (492 - ph_h as i32) / 2;

            draw_text_mut(
                &mut canvas,
                Rgb([100, 116, 139]),
                ph_x,
                ph_y,
                ph_scale,
                &self.regular_font,
                ph_text,
            );
        }

        self.draw_border(&mut canvas, 20, 20, 570, 492, Rgb([203, 213, 225]));

        let map_rect = Rect::at(610, 20).of_size(570, 492);
        draw_filled_rect_mut(&mut canvas, map_rect, Rgb([255, 255, 255]));

        let map_drawn = match self.fetch_map_image(tree.lat, tree.lon).await {
            Ok(bytes) => match self.decode_and_crop_image(&bytes, 570, 492) {
                Ok(img) => {
                    imageops::overlay(&mut canvas, &img, 610, 20);
                    true
                }
                Err(e) => {
                    warn!("Failed to decode/crop map image: {:?}. Falling back to placeholder graphic.", e);
                    false
                }
            },
            Err(e) => {
                warn!(
                    "Failed to fetch map image: {:?}. Falling back to placeholder graphic.",
                    e
                );
                false
            }
        };

        if !map_drawn {
            draw_filled_rect_mut(&mut canvas, map_rect, Rgb([226, 232, 240]));

            let mp_text = "Map Unavailable";
            let mp_scale = PxScale::from(24.0);
            let (mp_w, mp_h) = text_size(mp_scale, &self.regular_font, mp_text);
            let mp_x = 610 + (570 - mp_w as i32) / 2;
            let mp_y = 20 + (492 - mp_h as i32) / 2;

            draw_text_mut(
                &mut canvas,
                Rgb([100, 116, 139]),
                mp_x,
                mp_y,
                mp_scale,
                &self.regular_font,
                mp_text,
            );
        }

        self.draw_border(&mut canvas, 610, 20, 570, 492, Rgb([203, 213, 225]));

        let species = &tree.species;
        let address = match &tree.address {
            Some(addr) => addr.as_str(),
            None => {
                let s = format!("Lat: {:.4}, Lon: {:.4}", tree.lat, tree.lon);
                let leaked: &'static str = Box::leak(s.into_boxed_str());
                leaked
            }
        };

        debug!(
            "Compositing card for tree ID {}: species='{}', address='{}'",
            tree.id, species, address
        );

        draw_text_mut(
            &mut canvas,
            Rgb([15, 23, 42]),
            20,
            515,
            PxScale::from(48.0),
            &self.italic_font,
            species,
        );

        draw_text_mut(
            &mut canvas,
            Rgb([71, 85, 105]),
            20,
            575,
            PxScale::from(32.0),
            &self.regular_font,
            address,
        );

        debug!(
            "Encoding card canvas to JPEG ({}x{})...",
            canvas.width(),
            canvas.height()
        );

        let mut jpeg_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut jpeg_bytes);

        canvas
            .write_to(&mut cursor, image::ImageFormat::Jpeg)
            .map_err(|e| {
                error!("Failed to encode canvas to JPEG: {:?}", e);
                Error::ImageResize
            })?;

        info!(
            "Successfully generated OpenGraph card for tree ID {} in {} bytes",
            tree.id,
            jpeg_bytes.len()
        );

        Ok(jpeg_bytes)
    }

    fn decode_and_crop_image(&self, bytes: &[u8], target_w: u32, target_h: u32) -> Result<RgbImage> {
        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|_| Error::BadImage)?;
        let img = reader.decode().map_err(|_| Error::BadImage)?;

        let rgb_img = img.to_rgb8();
        let src_w = rgb_img.width() as f64;
        let src_h = rgb_img.height() as f64;
        let target_aspect = target_w as f64 / target_h as f64;
        let src_aspect = src_w / src_h;

        let (crop_w, crop_h, crop_x, crop_y) = if src_aspect > target_aspect {
            let h = src_h;
            let w = src_h * target_aspect;
            let x = (src_w - w) / 2.0;
            let y = 0.0;
            (w as u32, h as u32, x as u32, y as u32)
        } else {
            let w = src_w;
            let h = src_w / target_aspect;
            let x = 0.0;
            let y = (src_h - h) / 2.0;
            (w as u32, h as u32, x as u32, y as u32)
        };

        let cropped = imageops::crop_imm(&rgb_img, crop_x, crop_y, crop_w, crop_h).to_image();
        let resized = imageops::resize(
            &cropped,
            target_w,
            target_h,
            imageops::FilterType::Lanczos3,
        );

        Ok(resized)
    }

    async fn fetch_map_image(&self, lat: f64, lon: f64) -> Result<Vec<u8>> {
        let key = self
            .secrets
            .maptiler_key
            .as_deref()
            .or_else(|| self.config.maptiler_key.as_deref())
            .unwrap_or_default();

        debug!(
            "Fetching static map for tree ID (lat: {}, lon: {}) from MapTiler...",
            lat, lon
        );
        debug!("MapTiler API key present: {}", !key.is_empty());

        if key.is_empty() {
            warn!("MapTiler API key is not configured.");
            return Err(Error::FileDownload);
        }

        let url = format!(
            "https://api.maptiler.com/maps/streets-v2/static/{},{},17/570x492@2x.png?key={}&markers={},{}",
            lon, lat, key, lon, lat
        );

        let resp = self.http.get(&url).send().await.map_err(|e| {
            warn!("HTTP request to MapTiler failed: {:?}", e);
            Error::FileDownload
        })?;

        let status = resp.status();
        let bytes = resp.bytes().await.map_err(|e| {
            warn!("Failed to read MapTiler response bytes: {:?}", e);
            Error::FileDownload
        })?;

        debug!(
            "MapTiler HTTP response status: {}, size: {} bytes",
            status,
            bytes.len()
        );

        if !status.is_success() {
            warn!("MapTiler returned non-success status: {}", status);
            return Err(Error::FileDownload);
        }

        Ok(bytes.to_vec())
    }

    fn draw_border(&self, canvas: &mut RgbImage, x: u32, y: u32, w: u32, h: u32, color: Rgb<u8>) {
        for px in x..x + w {
            canvas.put_pixel(px, y, color);
            canvas.put_pixel(px, y + h - 1, color);
        }

        for py in y..y + h {
            canvas.put_pixel(x, py, color);
            canvas.put_pixel(x + w - 1, py, color);
        }
    }
}

impl Injectable for TreeCardService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let regular_font = FontArc::try_from_vec(REGULAR_FONT_BYTES.to_vec()).map_err(|e| {
            error!("Failed to parse regular font: {:?}", e);
            Error::BadImage
        })?;
        let italic_font = FontArc::try_from_vec(ITALIC_FONT_BYTES.to_vec()).map_err(|e| {
            error!("Failed to parse italic font: {:?}", e);
            Error::BadImage
        })?;

        Ok(Self {
            config: ctx.config(),
            secrets: ctx.secrets(),
            storage: ctx.storage(),
            http: reqwest::Client::new(),
            regular_font,
            italic_font,
        })
    }
}
