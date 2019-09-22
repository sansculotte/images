#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate image;

use std::fs::File;
use std::path::PathBuf;
use image::{FilterType, PNG, JPEG};
use rocket::{Request, response::content, response::NamedFile};


const IMAGE_DIR: &str = "images/";


#[get("/<domain>/<id>")]
fn images(domain: String, id: String) -> Option<NamedFile> {
    let filename = format!(
        "{path}/{domain}/{id}.jpg",
        path=IMAGE_DIR,
        domain=domain,
        id=id
    );
    NamedFile::open(&filename).ok()
}

#[get("/<domain>/thumb/<id>")]
fn scaled(domain: String, id: String)
    -> Option<NamedFile>
{
    let format = "thumb";
    let mut cached: PathBuf = [
        IMAGE_DIR,
        domain.as_str(),
        format,
        id.as_str()
    ].iter().collect();
    cached.set_extension("jpg");
    let f = NamedFile::open(&cached);
    match f {
        Ok(file) => Some(file),
        Err(_error) => {
            let filename = format!(
                "{path}/{domain}/{id}.jpg",
                path=IMAGE_DIR,
                domain=domain,
                id=id
            );
            let i = image::open(&filename);
            match i {
                Ok(img) => {
                    let scaled = img.resize_to_fill(280, 180, FilterType::Nearest);
                    let mut output = File::create(&cached).unwrap();
                    scaled.write_to(&mut output, JPEG).unwrap();
                    NamedFile::open(&cached).ok()
                },
                Err(_error) => None
            }
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
        .mount("/", routes![images, scaled])
        .register(catchers![not_found])
        .launch();
}
