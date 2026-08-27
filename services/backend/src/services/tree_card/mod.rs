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
use log::warn;
use std::io::Cursor;
use std::sync::Arc;
use tokio::fs;

pub struct TreeCardService {
    config: Arc<Config>,
    secrets: Arc<Secrets>,
    storage: Arc<FileBucket>,
    http: reqwest::Client,
}

impl TreeCardService {
    pub async fn get_card(&self, tree: &Tree) -> Result<Vec<u8>> {
        let thumbnail_id = match tree.thumbnail_id {
            Some(id) => id,
            None => 0,
        };
        let cache_path = format!(
            "var/cache/cards/{}_{}_{}.jpg",
            tree.id, thumbnail_id, tree.images_updated_at
        );

        if fs::metadata(&cache_path).await.is_ok() {
            if let Ok(data) = fs::read(&cache_path).await {
                return Ok(data);
            }
        }

        let _ = fs::create_dir_all("var/cache/cards").await;

        let regular_font = self
            .load_font(
                "NotoSans-Regular.ttf",
                "https://raw.githubusercontent.com/google/fonts/main/ofl/notosans/NotoSans-Regular.ttf",
            )
            .await?;

        let semibold_font = self
            .load_font(
                "NotoSans-SemiBold.ttf",
                "https://raw.githubusercontent.com/google/fonts/main/ofl/notosans/NotoSans-SemiBold.ttf",
            )
            .await?;

        let mut canvas = RgbImage::from_pixel(1200, 630, Rgb([244, 246, 244]));

        let header_left = "TREES OF YEREVAN";
        let header_right = "yerevan.treemaps.app";
        let header_scale = PxScale::from(18.0);
        let right_scale = PxScale::from(16.0);

        draw_text_mut(
            &mut canvas,
            Rgb([30, 41, 59]),
            110,
            38,
            header_scale,
            &semibold_font,
            header_left,
        );

        let (right_w, _) = text_size(right_scale, &regular_font, header_right);
        let right_x = 1200 - 110 - right_w as i32;

        draw_text_mut(
            &mut canvas,
            Rgb([100, 116, 139]),
            right_x,
            40,
            right_scale,
            &regular_font,
            header_right,
        );

        let left_block_rect = Rect::at(110, 90).of_size(430, 430);
        draw_filled_rect_mut(&mut canvas, left_block_rect, Rgb([255, 255, 255]));

        let photo_drawn = if let Some(thumb_id) = tree.thumbnail_id {
            match self.storage.read_file(thumb_id).await {
                Ok(bytes) => match self.decode_and_crop_image(&bytes, 430) {
                    Ok(img) => {
                        imageops::overlay(&mut canvas, &img, 110, 90);
                        true
                    }
                    Err(_) => false,
                },
                Err(_) => false,
            }
        } else {
            false
        };

        if !photo_drawn {
            let placeholder_rect = Rect::at(110, 90).of_size(430, 430);
            draw_filled_rect_mut(&mut canvas, placeholder_rect, Rgb([226, 232, 240]));

            let ph_text = "No Photo Available";
            let ph_scale = PxScale::from(22.0);
            let (ph_w, ph_h) = text_size(ph_scale, &regular_font, ph_text);
            let ph_x = 110 + (430 - ph_w as i32) / 2;
            let ph_y = 90 + (430 - ph_h as i32) / 2;

            draw_text_mut(
                &mut canvas,
                Rgb([100, 116, 139]),
                ph_x,
                ph_y,
                ph_scale,
                &regular_font,
                ph_text,
            );
        }

        self.draw_border(&mut canvas, 110, 90, 430, 430, Rgb([203, 213, 225]));

        let map_rect = Rect::at(660, 90).of_size(430, 430);
        draw_filled_rect_mut(&mut canvas, map_rect, Rgb([255, 255, 255]));

        let map_drawn = match self.fetch_map_image(tree.lat, tree.lon).await {
            Ok(bytes) => match self.decode_and_crop_image(&bytes, 430) {
                Ok(img) => {
                    imageops::overlay(&mut canvas, &img, 660, 90);
                    true
                }
                Err(_) => false,
            },
            Err(_) => false,
        };

        if !map_drawn {
            draw_filled_rect_mut(&mut canvas, map_rect, Rgb([226, 232, 240]));

            let mp_text = "Map Unavailable";
            let mp_scale = PxScale::from(22.0);
            let (mp_w, mp_h) = text_size(mp_scale, &regular_font, mp_text);
            let mp_x = 660 + (430 - mp_w as i32) / 2;
            let mp_y = 90 + (430 - mp_h as i32) / 2;

            draw_text_mut(
                &mut canvas,
                Rgb([100, 116, 139]),
                mp_x,
                mp_y,
                mp_scale,
                &regular_font,
                mp_text,
            );
        }

        self.draw_border(&mut canvas, 660, 90, 430, 430, Rgb([203, 213, 225]));

        let species = &tree.species;
        let address = match &tree.address {
            Some(addr) => addr.as_str(),
            None => {
                let s = format!("Lat: {:.4}, Lon: {:.4}", tree.lat, tree.lon);
                let leaked: &'static str = Box::leak(s.into_boxed_str());
                leaked
            }
        };

        draw_text_mut(
            &mut canvas,
            Rgb([15, 23, 42]),
            110,
            540,
            PxScale::from(26.0),
            &semibold_font,
            species,
        );

        draw_text_mut(
            &mut canvas,
            Rgb([71, 85, 105]),
            110,
            575,
            PxScale::from(18.0),
            &regular_font,
            address,
        );

        let mut jpeg_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut jpeg_bytes);

        canvas
            .write_to(&mut cursor, image::ImageFormat::Jpeg)
            .map_err(|_| Error::ImageResize)?;

        let _ = fs::write(&cache_path, &jpeg_bytes).await;

        Ok(jpeg_bytes)
    }

    async fn load_font(&self, filename: &str, url: &str) -> Result<FontArc> {
        let font_dir = "var/fonts";
        let font_path = format!("{}/{}", font_dir, filename);

        let bytes = if fs::metadata(&font_path).await.is_ok() {
            fs::read(&font_path)
                .await
                .map_err(|_| Error::FileNotFound)?
        } else {
            let _ = fs::create_dir_all(font_dir).await;

            match self.http.get(url).send().await {
                Ok(resp) => {
                    if let Ok(b) = resp.bytes().await {
                        let _ = fs::write(&font_path, &b).await;
                        b.to_vec()
                    } else {
                        return Err(Error::FileDownload);
                    }
                }
                Err(_) => {
                    return Err(Error::FileDownload);
                }
            }
        };

        FontArc::try_from_vec(bytes).map_err(|_| Error::BadImage)
    }

    fn decode_and_crop_image(&self, bytes: &[u8], target_size: u32) -> Result<RgbImage> {
        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|_| Error::BadImage)?;
        let img = reader.decode().map_err(|_| Error::BadImage)?;

        let rgb_img = img.to_rgb8();
        let w = rgb_img.width();
        let h = rgb_img.height();
        let min_dim = std::cmp::min(w, h);
        let x = (w - min_dim) / 2;
        let y = (h - min_dim) / 2;

        let cropped = imageops::crop_imm(&rgb_img, x, y, min_dim, min_dim).to_image();
        let resized = imageops::resize(
            &cropped,
            target_size,
            target_size,
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

        if key.is_empty() {
            warn!("MapTiler API key is not configured.");
            return Err(Error::FileDownload);
        }

        let url = format!(
            "https://api.maptiler.com/maps/streets-v2/static/{},{},17/430x430@2x.png?key={}&markers={},{}",
            lon, lat, key, lon, lat
        );

        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|_| Error::FileDownload)?;

        if !resp.status().is_success() {
            return Err(Error::FileDownload);
        }

        let bytes = resp.bytes().await.map_err(|_| Error::FileDownload)?;

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
        Ok(Self {
            config: ctx.config(),
            secrets: ctx.secrets(),
            storage: ctx.storage(),
            http: reqwest::Client::new(),
        })
    }
}
