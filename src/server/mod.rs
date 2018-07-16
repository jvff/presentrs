mod notes;
mod slides;

mod server;

pub use self::notes::{Notes, NotesError};
pub use self::slides::{Slides, SlidesError};

pub use self::server::{Server, ServerStartError};
