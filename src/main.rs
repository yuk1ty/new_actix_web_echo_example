#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpResponse, HttpServer, middleware, Result, http};
use actix_rt::System;
use std::io;
use std::env;

#[get("/hc")]
fn hc() -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK).body("OK".as_bytes()))
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = System::new("actix-web-1.0-example");

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(hc)
    }).bind("127.0.0.1:3000")?.start();

    sys.run()
}
