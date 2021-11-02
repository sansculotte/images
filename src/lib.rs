extern crate image;

use std::env;
use std::path::PathBuf;
use self::image::{FilterType};


const IMAGE_DIR: &str = "images/";

fn image_dir() -> String {
        env::var("IMAGE_DIR").unwrap_or(String::from(IMAGE_DIR))
}

pub fn resize_and_crop_to(filename: &PathBuf, cached: &PathBuf, width: u32, height: u32) -> Option<()> {
    let i = image::open(filename);
    match i {
        Ok(img) => {
            let scaled = img.resize_to_fill(width, height, FilterType::CatmullRom);
            scaled.save(cached).ok()
        },
        Err(_error) => None
    }
}

pub fn get_filename(domain: String, name: String) -> PathBuf {
    let filename: PathBuf = [ image_dir(), domain, name ].iter().collect();
    filename
}

pub fn get_cache_filename(domain: String, name: String, format: String) -> PathBuf {
    let filename = [ image_dir(), domain, format, name ].iter().collect();
    filename
}
