const IMAGE_DIR: &str = "images/";
const THUMB_WIDTH: i64 = 280;
const THUMB_HEIGHT: i64 = 120;
const CACHE_LEVEL: &str = "Long";

use crate::cache::CacheLevel;

#[derive(Clone, Debug)]
pub struct Config {
    pub image_dir: String,
    pub thumb_width: u32,
    pub thumb_height: u32,
    pub cache_level: CacheLevel,
}

impl Config {
    pub fn from(rocket_config: &rocket::Config) -> Self {
        let image_dir: String =
            String::from(rocket_config.get_str("image_dir").unwrap_or(IMAGE_DIR));
        let thumb_width: u32 = rocket_config.get_int("thumb_width").unwrap_or(THUMB_WIDTH) as u32;
        let thumb_height: u32 = rocket_config.get_int("thumb_height").unwrap_or(THUMB_HEIGHT) as u32;
        let cache_level =
            CacheLevel::parse(rocket_config.get_str("cache_level").unwrap_or(CACHE_LEVEL));
        Self {
            image_dir: image_dir,
            thumb_width: thumb_width,
            thumb_height: thumb_height,
            cache_level: cache_level,
        }
    }
}
