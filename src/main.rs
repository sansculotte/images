#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::fs::File;
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
        .mount("/", routes![images])
        .register(catchers![not_found])
        .launch();
}
