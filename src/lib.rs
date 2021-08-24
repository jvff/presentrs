#[cfg(not(target_family = "wasm"))]
mod server;

#[cfg(not(target_family = "wasm"))]
pub use crate::server::{Notes, NotesError, Slides, SlidesError};
