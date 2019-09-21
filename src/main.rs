#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate image;

use std::fs::File;
use image::{FilterType, PNG, JPEG};
use rocket::{Request, response::content, data::Data};


const IMAGE_DIR: &str = "images/";


#[get("/<domain>/<id>")]
fn images(domain: String, id: String) -> Option<content::Plain<File>> {
    let filename = format!(
        "{path}/{domain}/{id}.jpg",
        path=IMAGE_DIR,
        domain=domain,
        id=id
    );
    File::open(&filename).map(|f| content::Plain(f)).ok()
}

#[get("/<domain>/thumb/<id>")]
fn scaled(domain: String, id: String)
    -> Option<content::Plain<File>>
{
    let format = "thumb";
    let cacheFilename = format!(
        "{path}/{domain}/{format}/{id}.jpg",
        path=IMAGE_DIR,
        domain=domain,
        format=format,
        id=id
    );
    let f = File::open(&cacheFilename);
    match f {
        Ok(file) => Some(content::Plain(file)),
        Err(error) => {
            let filename = format!(
                "{path}/{domain}/{id}.jpg",
                path=IMAGE_DIR,
                domain=domain,
                id=id
            );
            let i = image::open(filename);
            match i {
                Ok(img) => {
                    let scaled = img.resize_to_fill(280, 180, FilterType::Nearest);
                    let mut output = File::create(cacheFilename).unwrap();
                    scaled.write_to(&mut output, JPEG).unwrap();
                    output.sync_all();
                    Some(content::Plain(output))
                },
                Err(error) => None 
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
