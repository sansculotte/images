extern crate image;
extern crate rocket;

use std::path::PathBuf;
use self::image::imageops::{FilterType};


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

pub fn get_filename(image_dir: String, domain: String, name: String) -> PathBuf {
    let filename: PathBuf = [ image_dir, domain, name ].iter().collect();
    filename
}

pub fn get_cache_filename(image_dir: String, domain: String, name: String, format: String) -> PathBuf {
    let filename = [ image_dir, domain, format, name ].iter().collect();
    filename
}
