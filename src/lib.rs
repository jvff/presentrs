#[cfg(target_family = "wasm")]
mod client;

#[cfg(not(target_family = "wasm"))]
mod server;

#[cfg(target_family = "wasm")]
pub use crate::client::Presentrs;

#[cfg(not(target_family = "wasm"))]
pub use crate::server::{Notes, NotesError, Slides, SlidesError};
