#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{Request, response::content, response::NamedFile};

mod cache;
use cache::Cached;

mod lib;
use lib::{
    resize_and_crop_to,
    get_filename,
    get_cache_filename
};


#[get("/<domain>/<image>")]
fn original(domain: String, image: String) -> Cached<Option<NamedFile>> {
    let filename = get_filename(domain.as_str(), image.as_str());
    Cached::long(NamedFile::open(filename.as_os_str()).ok())
}

#[get("/<domain>/thumb/<image>")]
fn scaled(domain: String, image: String) -> Cached<Option<NamedFile>> {
    let format = "thumb";
    let cached = get_cache_filename(domain.as_str(), image.as_str(), format);
    let f = NamedFile::open(&cached);
    match f {
        Ok(file) => Cached::long(Some(file)),
        Err(_error) => {
            let filename = get_filename(domain.as_str(), image.as_str());
            resize_and_crop_to(&filename, &cached, 280, 180);
            Cached::long(NamedFile::open(cached).ok())
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

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![original, scaled])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn hello_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn image_not_found() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/test/xxx.jpg").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn image() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/test/235.jpg").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JPEG));
    }

    #[test]
    fn thumb_not_found() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/test/thumb/xxx.jpg").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn thumb() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/test/thumb/235.jpg").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JPEG));
    }

}
