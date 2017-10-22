extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world rust")))
    }

    let _server = Iron::new(hello_world).http("localhost:8080").unwrap();
    println!("Lisning on port 8080");
}
