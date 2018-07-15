#[cfg(feature = "client")]
extern crate stdweb;
#[cfg(feature = "client")]
#[macro_use]
extern crate yew;

#[cfg(feature = "server")]
extern crate actix_web;
#[cfg(feature = "server")]
extern crate comrak;
#[cfg(feature = "server")]
#[macro_use]
extern crate failure;
#[cfg(feature = "server")]
extern crate html5ever;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::Presentrs;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::{Notes, NotesError, Server, ServerStartError};
