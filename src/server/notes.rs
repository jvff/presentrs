use std::borrow::BorrowMut;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::{fs, io};

use comrak::{markdown_to_html, ComrakOptions};
use html5ever::driver::ParseOpts;
use html5ever::interface::{Attribute, QualName};
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::serialize::SerializeOpts;
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};

#[derive(Debug)]
pub struct Notes {
    output: String,
    style: Option<String>,
}

impl Notes {
    pub fn from_markdown<P: AsRef<Path>>(
        markdown_file: P,
    ) -> Result<Self, NotesError> {
        let input =
            fs::read_to_string(markdown_file).map_err(NotesError::LoadError)?;
        let html = markdown_to_html(&input, &ComrakOptions::default());
        let output = format!("<div>{}</div>", html);

        Ok(Notes {
            output,
            style: None,
        })
    }

    pub fn animate_steps(&mut self) -> Result<&mut Self, NotesError> {
        let html_dom = parse_document(RcDom::default(), ParseOpts::default())
            .from_utf8()
            .read_from(&mut self.output.as_bytes())
            .map_err(NotesError::AnimateStepsError)?;
        let mut document_nodes = html_dom.document.children.borrow_mut();
        let html = document_nodes[0].borrow_mut();
        let mut html_nodes = html.children.borrow_mut();
        let mut body = html_nodes[1].borrow_mut();

        let mut slide = 0;
        let mut step = 0;

        Self::animate_steps_on(&mut slide, &mut step, &mut body);

        let mut style = String::new();

        for i in 1..=slide {
            for j in 1..=slide {
                if i != j {
                    style.push_str(&format!(
                        "div.current-slide-{} .slide-{} {{ display: none; }}",
                        i, j
                    ));
                }
            }
        }

        let mut output = Cursor::new(Vec::new());

        serialize(&mut output, body, SerializeOpts::default())
            .map_err(NotesError::AnimateStepsError)?;

        self.output = String::from_utf8_lossy(output.get_ref()).to_string();
        self.style = Some(style);

        Ok(self)
    }

    fn animate_steps_on(
        slide: &mut usize,
        step: &mut usize,
        node: &mut Handle,
    ) {
        let node = node.borrow_mut();

        match node.data {
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                if &name.local == "h2" || &name.local == "h3" {
                    *slide += 1;
                    *step = 0;
                } else if &name.local == "li" {
                    *step += 1;
                }

                if *slide > 0 {
                    let step_classes = if *step > 0 {
                        format!("slide-{} slide-step-{}", slide, step)
                    } else {
                        format!("slide-{}", slide)
                    };

                    let mut attributes = attrs.borrow_mut();
                    let new_attribute = {
                        let attribute = attributes
                            .iter_mut()
                            .find(|attribute| &attribute.name.local == "class");

                        match attribute {
                            Some(class_attribute) => {
                                let new_value = format!(
                                    "{} {}",
                                    class_attribute.value, step_classes,
                                );

                                class_attribute.value = new_value.into();

                                None
                            }
                            None => {
                                let namespace = "".into();
                                let name = QualName::new(
                                    None,
                                    namespace,
                                    "class".into(),
                                );
                                let value = step_classes.into();

                                Some(Attribute { name, value })
                            }
                        }
                    };

                    if let Some(attribute) = new_attribute {
                        attributes.push(attribute);
                    }
                }
            }
            _ => {}
        };

        for child in node.children.borrow_mut().iter_mut() {
            Self::animate_steps_on(slide, step, child);
        }
    }

    pub fn generate_html(&mut self) -> Result<(), NotesError> {
        fs::write(PathBuf::from("static/notes.html"), &self.output)
            .map_err(NotesError::GenerateHtmlError)?;

        if let Some(ref style) = self.style {
            fs::write(PathBuf::from("static/notes.css"), &style)
                .map_err(NotesError::GenerateHtmlError)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Fail)]
pub enum NotesError {
    #[fail(display = "Failed to load notes")]
    LoadError(#[cause] io::Error),
    #[fail(display = "Failed to animate steps for notes")]
    AnimateStepsError(#[cause] io::Error),
    #[fail(display = "Failed to generate HTML file for notes")]
    GenerateHtmlError(#[cause] io::Error),
}
