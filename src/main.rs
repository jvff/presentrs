extern crate actix_web;

use actix_web::{server, App, HttpRequest};

fn index(_: HttpRequest) -> &'static str {
    "Hello, world!"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
