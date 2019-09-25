extern crate image;

use std::fs::File;
use std::path::PathBuf;
use self::image::{FilterType, JPEG};


const IMAGE_DIR: &str = "images/";


pub fn resize_and_crop_to(filename: &PathBuf, cached: &PathBuf, width: u32, height: u32) -> Option<()> {
    let i = image::open(filename);
    match i {
        Ok(img) => {
            let scaled = img.resize_to_fill(width, height, FilterType::CatmullRom);
            let mut output = File::create(cached).expect("Could not open cache file");
            scaled.write_to(&mut output, JPEG).ok()
        },
        Err(_error) => None
    }
}

pub fn get_filename(domain: &str, name: &str) -> PathBuf {
    let filename: PathBuf = [ IMAGE_DIR, domain, name ].iter().collect();
    filename
}

pub fn get_cache_filename(domain: &str, name: &str, format: &str) -> PathBuf {
    let filename = [ IMAGE_DIR, domain, format, name ].iter().collect();
    filename
}
