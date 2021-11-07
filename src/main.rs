#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{Request, State};
use rocket::response::{content, NamedFile};
use rocket::fairing::AdHoc;

mod config;
use config::Config;

mod cache;
use cache::Cache;

mod lib;
use lib::{
    resize_and_crop_to,
    get_filename,
    get_cache_filename,
};


#[get("/<domain>/<image>")]
fn original(domain: String, image: String, config: State<Config>) -> Cache<Option<NamedFile>> {
    let filename = get_filename(config.image_dir.clone(), domain, image);
    Cache::deliver(config.cache_level.clone(), NamedFile::open(filename.as_os_str()).ok())
}

#[get("/<domain>/thumb/<image>")]
fn scaled(domain: String, image: String, config: State<Config>) -> Cache<Option<NamedFile>> {
    let width = config.thumb_width;
    let height = config.thumb_height;
    let format = String::from("thumb");
    let cached = get_cache_filename(config.image_dir.clone(), domain.clone(), image.clone(), format);
    let f = NamedFile::open(&cached);
    match f {
        Ok(file) => Cache::deliver(config.cache_level.clone(), Some(file)),
        Err(_error) => {
            let filename = get_filename(config.image_dir.clone(), domain, image);
            resize_and_crop_to(&filename, &cached, width, height);
            Cache::deliver(config.cache_level.clone(), NamedFile::open(cached).ok())
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
        .attach(
            AdHoc::on_attach("Config", |rocket| {
                let rocket_config = rocket.config().clone();
                let config = Config::from(&rocket_config);
                Ok(rocket.manage(config))
            })
        )
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
    use config::Config;

    #[test]
    fn test_config() {
        let rocket_config = rocket::Config::new();
        rocket_config.set("image_dir", "images");
        let config = Config::from(&rocket_config);
        assert_eq!(config.image_dir, "images")
    }

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
