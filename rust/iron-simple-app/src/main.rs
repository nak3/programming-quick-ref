extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;

use std::path::Path;
use iron::prelude::*;
use iron::headers::ContentType;
use iron::status;
use mount::Mount;
use router::Router;
use staticfile::Static;
use tera::{Tera, Context};


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!("templates/*");
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions
        .get::<Router>()
        .unwrap()
        .find("query")
        .unwrap_or("/");

    let mut context = Context::new();
    context.add("query", &query.to_string());
    let html = TEMPLATES.render("index.html", &context).unwrap();
    Ok(Response::with((ContentType::html().0, status::Ok, html)))
}

fn main() {

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/css", Static::new(Path::new("assets/css")));
    print!("server is running at localhost:3000\n");

    Iron::new(mount).http("localhost:3000").unwrap();
}
