use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

use derive_more::{Display, Error};
use html5ever::driver::ParseOpts;
use html5ever::interface::Attribute;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::serialize::{SerializeOpts, TraversalScope};
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};

use super::notes::Notes;

pub struct Slides {
    slides: Vec<String>,
}

impl Slides {
    pub fn from_notes(notes: &Notes) -> Result<Slides, SlidesError> {
        let html = notes.html_str().clone();
        let html_dom = parse_document(RcDom::default(), ParseOpts::default())
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .map_err(SlidesError::FromNotesError)?;
        let document_nodes = html_dom.document.children.borrow();
        let html = &document_nodes[0];
        let html_nodes = html.children.borrow();
        let body = &html_nodes[1];
        let body_nodes = body.children.borrow();
        let container = &body_nodes[0];

        let mut slide_map = HashMap::new();

        for child in container.children.borrow().iter() {
            Self::extract_slides_from(child, &mut slide_map)?;
        }

        let slide_count = *slide_map.keys().max().unwrap_or(&0);
        let mut slides = vec![String::from("<div></div>"); slide_count];

        for (number, contents) in slide_map {
            slides[number - 1] = format!("<div>{}</div>", contents);
        }

        Ok(Slides { slides })
    }

    fn extract_slides_from(
        node: &Handle,
        slide_map: &mut HashMap<usize, String>,
    ) -> Result<(), SlidesError> {
        match node.data {
            NodeData::Element { ref attrs, .. } => {
                if let Some(slide_number) =
                    Self::current_slide_of(&attrs.borrow())
                {
                    let mut slide = Vec::new();
                    let options = SerializeOpts {
                        traversal_scope: TraversalScope::IncludeNode,
                        ..SerializeOpts::default()
                    };

                    serialize(&mut slide, node, options)
                        .map_err(SlidesError::FromNotesError)?;

                    let slide_string = String::from_utf8_lossy(&slide);

                    slide_map
                        .entry(slide_number)
                        .and_modify(|slide| slide.push_str(&slide_string))
                        .or_insert_with(|| slide_string.to_string());
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn current_slide_of(attributes: &[Attribute]) -> Option<usize> {
        attributes
            .iter()
            .find(|attribute| &attribute.name.local == "class")
            .and_then(|class_attribute| {
                class_attribute
                    .value
                    .split_whitespace()
                    .find(|class_value| {
                        class_value.starts_with("slide-")
                            && !class_value.starts_with("slide-step-")
                    })
                    .and_then(|slide_value| {
                        let chars_to_skip = "slide-".len();
                        let slide_number = &slide_value[chars_to_skip..];

                        slide_number.parse().ok()
                    })
            })
    }

    pub fn load_from<P: AsRef<Path>>(
        &mut self,
        directory: P,
    ) -> Result<(), SlidesError> {
        let files: Vec<_> = fs::read_dir(directory)
            .and_then(|entries| entries.collect())
            .map_err(SlidesError::SlidesDirectoryError)?;

        let slide_files = files.iter().filter_map(Self::parse_slide_path);

        for (slide_number, slide_file) in slide_files {
            let slide_contents =
                fs::read_to_string(slide_file).map_err(SlidesError::LoadSlide)?;

            if slide_number > self.slides.len() {
                self.slides.resize(slide_number, String::new());
            }

            self.slides[slide_number - 1] = slide_contents;
        }

        Ok(())
    }

    fn parse_slide_path(dir_entry: &DirEntry) -> Option<(usize, PathBuf)> {
        let path = dir_entry.path();

        if path.extension() == Some(OsStr::new("html")) {
            let slide_number = path
                .file_stem()
                .map(|stem| stem.to_string_lossy().parse())
                .and_then(Result::ok);

            slide_number.map(|number| (number, path.to_owned()))
        } else {
            None
        }
    }

    pub fn write_to<P: AsRef<Path>>(
        &self,
        output_dir: P,
    ) -> Result<(), SlidesError> {
        let output_dir = output_dir.as_ref();
        let mut slide_number = 1;

        for slide in &self.slides {
            let slide_path = output_dir.join(format!("{}.html", slide_number));

            fs::write(slide_path, slide).map_err(SlidesError::WriteError)?;

            slide_number += 1;
        }

        Ok(())
    }
}

#[derive(Debug, Display, Error)]
pub enum SlidesError {
    #[display(fmt = "Failed to parse slides from notes")]
    FromNotesError(io::Error),
    #[display(fmt = "Failed to read slides from directory")]
    SlidesDirectoryError(io::Error),
    #[display(fmt = "Failed to read slide file")]
    LoadSlide(io::Error),
    #[display(fmt = "Failed to write slide")]
    WriteError(io::Error),
}
