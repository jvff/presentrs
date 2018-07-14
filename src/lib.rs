extern crate actix_web;
#[macro_use]
extern crate failure;

mod server;

pub use server::{Server, ServerStartError};
