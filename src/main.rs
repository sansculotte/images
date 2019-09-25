#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::fs::File;
use rocket::{Request, response::content, response::NamedFile};

mod lib;
use lib::{
    resize_and_crop_to,
    get_filename,
    get_cache_filename
};


#[get("/<domain>/<image>")]
fn original(domain: String, image: String) -> Option<NamedFile> {
    let filename = get_filename(domain.as_str(), image.as_str());
    NamedFile::open(filename.as_os_str()).ok()
}

#[get("/<domain>/thumb/<image>")]
fn scaled(domain: String, image: String) -> Option<NamedFile> {
    let format = "thumb";
    let cached = get_cache_filename(domain.as_str(), image.as_str(), format);
    let f = NamedFile::open(&cached);
    match f {
        Ok(file) => Some(file),
        Err(_error) => {
            let filename = get_filename(domain.as_str(), image.as_str());
            resize_and_crop_to(&filename, &cached, 280, 180)?;
            NamedFile::open(cached).ok()
       }
    }
}

#[catch(404)]
fn not_found(request: &Request) -> content::Html<String> {
    let html = format!(
        "<p>{} not found</p>",
         request.uri()
    );
    content::Html(html)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![original, scaled])
        .register(catchers![not_found])
        .launch();
}
