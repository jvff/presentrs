use std::path::{Path, PathBuf};
use std::{fs, io};

use comrak::{markdown_to_html, ComrakOptions};

#[derive(Debug)]
pub struct Notes {
    output: String,
}

impl Notes {
    pub fn from_markdown<P: AsRef<Path>>(
        markdown_file: P,
    ) -> Result<Self, NotesError> {
        let input =
            fs::read_to_string(markdown_file).map_err(NotesError::LoadError)?;
        let output = markdown_to_html(&input, &ComrakOptions::default());

        Ok(Notes { output })
    }

    pub fn generate_html(&self) -> Result<(), NotesError> {
        let mut output = String::from("<div>");

        output.push_str(&self.output);
        output.push_str("</div>");

        fs::write(PathBuf::from("static/notes.html"), &output)
            .map_err(NotesError::GenerateHtmlError)
    }
}

#[derive(Debug, Fail)]
pub enum NotesError {
    #[fail(display = "Failed to load notes")]
    LoadError(#[cause] io::Error),
    #[fail(display = "Failed to HTML file for notes")]
    GenerateHtmlError(#[cause] io::Error),
}
