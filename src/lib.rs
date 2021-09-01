#[cfg(target_family = "wasm")]
mod client;

#[cfg(not(target_family = "wasm"))]
mod server;

#[cfg(target_family = "wasm")]
pub use crate::client::{Presentrs, Properties};

#[cfg(not(target_family = "wasm"))]
pub use crate::server::{Notes, NotesError, Presentrs, Slides, SlidesError};
