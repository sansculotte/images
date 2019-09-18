#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


use rocket::{Request, response::content, data::Data};


#[get("/")]
fn index() -> &'static str {
    "Artfacts image server"
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
        .mount("/", routes![index])
        .register(catchers![not_found])
        .launch();
}
