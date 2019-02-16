extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::status;
use iron::{typemap, AfterMiddleware, BeforeMiddleware};
use time::precise_time_ns;
use router::Router;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn main() {
    let mut router = Router::new();

    router.get("/".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "It's root page")))
    }, "root");

    router.get("/hello".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world !")))
    }, "hello");

    router.get("/hello/again".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello again !")))
    }, "helloAgain");

    router.get("/error".to_string(), |_: &mut Request| {
        Ok(Response::with(status::BadRequest))
    }, "error");

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000");
}
