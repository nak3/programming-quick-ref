extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use std::collections::HashMap;

fn tmpl_handler<'a> (_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();
    // add data for render
    // name = {{ name }} in template
    // page_title = {{ page_title }}
    data.insert("name", "Test");
    data.insert("page_title", "page title!");
    res.render("app/views/index.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();
    server.get("/", tmpl_handler);
    server.listen("127.0.0.1:8080");
}
