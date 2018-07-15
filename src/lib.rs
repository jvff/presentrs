#[cfg(feature = "server")]
extern crate actix_web;
#[cfg(feature = "server")]
#[macro_use]
extern crate failure;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "server")]
pub use server::{Server, ServerStartError};
