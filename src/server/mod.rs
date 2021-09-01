mod notes;
mod presentrs;
mod slide_presenter;
mod slides;

pub use self::{
    notes::{Notes, NotesError},
    presentrs::Presentrs,
    slides::{Slides, SlidesError},
};
