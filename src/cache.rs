use rocket::{response::{self, Responder}, Request};

#[derive(Clone, Debug)]
pub enum CacheLevel {
    None,
    Short,
    Medium,
    Long,
}

impl CacheLevel {
    pub fn parse(s: &str) -> Self {
        match s {
            "None" => Self::None,
            "Short" => Self::Short,
            "Medium" => Self::Medium,
            "Long" => Self::Long,
            _ => Self::Long,
        }
    }
}


pub struct Cache<R>(R, &'static str);

impl<R> Cache<R> {

    pub fn deliver(cache_level: CacheLevel, r: R) -> Cache<R> {
        match cache_level {
            CacheLevel::Long => Self::long(r),
            CacheLevel::Medium => Self::medium(r),
            CacheLevel::Short => Self::short(r),
            CacheLevel::None => Self::none(r)
        }
    }

    const fn long(r: R) -> Cache<R> {
        // 7 days
        Self(r, "public, max-age=604800")
    }

    const fn medium(r: R) -> Cache<R> {
        // 24 hours
        Self(r, "public, max-age=86400")
    }

    const fn short(r: R) -> Cache<R> {
        // 2 hours
        Self(r, "public, max-age=7200")
    }

    const fn none(r: R) -> Cache<R> {
        Self(r, "no-store, max-age=0")
    }
}

impl<'r, R: Responder<'r>> Responder<'r> for Cache<R> {
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
