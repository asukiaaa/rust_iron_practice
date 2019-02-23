extern crate iron;
extern crate params;
#[macro_use] extern crate router;
extern crate time;

use handlebars_iron as hbs;
use hbs::{DirectorySource, HandlebarsEngine, Template};
use iron::prelude::*;
use iron::status;
use iron::{typemap, AfterMiddleware, BeforeMiddleware};
use mount::Mount;
use params::{Params, Value};
use router::Router;
use serde_json::{Value as Json};
use staticfile::Static;
use std::collections::HashMap;
use std::path::Path;
use time::precise_time_ns;
use handlebars::to_json;

struct Middleware;

impl typemap::Key for Middleware { type Value = u64; }

impl BeforeMiddleware for Middleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Middleware>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for Middleware {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<Middleware>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("{:?}", err);
        Err(err)
    }
}

fn create_data(page_template_path: String) -> HashMap<String, Json> {
    let mut data = HashMap::new();
    data.insert("page_template_path".to_string(), to_json(page_template_path));
    data
}

fn root_handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = create_data("index".to_string());
    data.insert("hello_url".to_string(), to_json(url_for!(req, "hello").to_string()));
    data.insert("hello_again_url".to_string(), to_json(url_for!(req, "hello_again").to_string()));
    data.insert(
        "hello_again_bob_url".to_string(),
        to_json(url_for!(req, "hello_again", "name" => "Bob").to_string())
    );
    resp.set_mut(Template::new("layouts/default", data)).set_mut(status::Ok);
    Ok(resp)
}

fn hello_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let data = create_data("hello".to_string());
    resp.set_mut(Template::new("layouts/default", data)).set_mut(status::Ok);
    Ok(resp)
}

fn hello_again_handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut data = create_data("hello_again".to_string());
    let params = req.get_ref::<Params>().unwrap();
    let name = match params.find(&["name"]) {
        Some(&Value::String(ref name)) => name,
        _ => ""
    };
    data.insert("name".to_string(), to_json(name));
    resp.set_mut(Template::new("layouts/default", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut router = Router::new();
    let mut hbse = HandlebarsEngine::new();
    hbse.handlebars_mut().set_strict_mode(true);
    hbse.add(Box::new(DirectorySource::new("./templates", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    router.get("/".to_string(), root_handler, "root");
    router.get("/hello".to_string(), hello_handler, "hello");
    router.get("/hello/again".to_string(), hello_again_handler, "hello_again");
    router.get("/error".to_string(), |_: &mut Request| {
        Ok(Response::with(status::BadRequest))
    }, "error");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/public", Static::new(Path::new("public")));

    let mut chain = Chain::new(mount);
    chain.link_before(Middleware);
    chain.link_after(hbse);
    chain.link_after(Middleware);
    if let Err(r) = Iron::new(chain).http("0.0.0.0:80") {
        panic!("{}", r);
    }
}
