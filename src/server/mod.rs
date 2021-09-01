mod notes;
mod presentrs;
mod slides;

pub use self::{
    notes::{Notes, NotesError},
    presentrs::Presentrs,
    slides::{Slides, SlidesError},
};
