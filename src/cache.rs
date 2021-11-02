use rocket::{response::{self, Responder}, Request};

pub struct Cached<R>(R, &'static str);

impl<R> Cached<R> {
    pub const fn long(r: R) -> Cached<R> {
        // 7 days
        Self(r, "public, max-age=604800")
    }

    pub const fn medium(r: R) -> Cached<R> {
        // 24 hours
        Self(r, "public, max-age=86400")
    }

    pub const fn short(r: R) -> Cached<R> {
        // 2 hours
        Self(r, "public, max-age=7200")
    }
}

impl<'r, R: Responder<'r>> Responder<'r> for Cached<R> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self.0.respond_to(req) {
            Ok(mut res) => {
                res.set_raw_header("Cache-Control", self.1);
                Ok(res)
            }
            e @ Err(_) => e,
        }
    }
}
