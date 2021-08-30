use std::collections::HashSet;
use std::{self, cmp};

use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

pub struct Slide {
    contents: Node,
    num_steps: usize,
    animated_elements: Vec<AnimatedElement>,
}

impl Slide {
    pub fn from_html(html: &str) -> Result<Self, String> {
        let window = web_sys::window().ok_or("Failed to access window")?;
        let document = window.document().ok_or("Window has no document")?;
        let parent_element =
            document.create_element("div").map_err(|error| {
                format!("Failed to create <div> element: {:?}", error)
            })?;

        parent_element.set_inner_html(html);

        let element = parent_element
            .first_element_child()
            .ok_or_else(|| "Invalid HTML to create a slide".to_owned())?;

        Ok(Self::new(element.into()))
    }

    pub fn new(contents: Node) -> Self {
        let mut animated_elements = Vec::new();
        let mut num_steps = 0;

        Self::load_contents(&contents, &mut num_steps, &mut animated_elements);

        Slide {
            contents,
            num_steps,
            animated_elements,
        }
    }

    fn load_contents(
        node: &Node,
        max_steps: &mut usize,
        animated_elements: &mut Vec<AnimatedElement>,
    ) {
        let maybe_element: Result<Element, _> = node.clone().dyn_into();

        if let Ok(element) = maybe_element {
            if let Some(animated_element) = AnimatedElement::try_from(element) {
                *max_steps =
                    cmp::max(*max_steps, animated_element.last_known_step());

                animated_elements.push(animated_element);
            }
        }

        let child_nodes = node.child_nodes();

        for index in 0..child_nodes.length() {
            if let Some(child) = child_nodes.get(index) {
                Self::load_contents(&child, max_steps, animated_elements);
            }
        }
    }

    pub fn animate_for_step(&mut self, step: usize) {
        for animated_element in self.animated_elements.iter_mut() {
            animated_element.animate_for_step(step);
        }
    }

    pub fn as_node(&self) -> Result<Node, String> {
        self.contents
            .clone_node_with_deep(true)
            .map_err(|_| "Failed to clone node tree".to_owned())
    }

    pub fn num_steps(&self) -> usize {
        self.num_steps
    }
}

#[derive(Clone, Debug)]
struct AnimatedElement {
    element: Element,
    class: Option<String>,
    steps: HashSet<usize>,
    always_show_after: Option<usize>,
    last_known_step: usize,
}

impl AnimatedElement {
    pub fn try_from(element: Element) -> Option<Self> {
        if let Some(step_spec) = element.get_attribute("data-slide-steps") {
            let class = element.get_attribute("class").map(|mut class| {
                if !class.ends_with(';') {
                    class.push(';');
                }
                class
            });

            let mut steps = HashSet::new();
            let mut always_show_after = None;
            let mut last_known_step = 0;

            for range in Self::step_ranges(step_spec.trim()) {
                match range {
                    (Some(start), Some(end)) => {
                        for step in start..=end {
                            steps.insert(step);
                        }

                        last_known_step = cmp::max(last_known_step, end);
                    }
                    (Some(start), None) => {
                        steps.insert(start);

                        last_known_step = cmp::max(last_known_step, start);

                        let previous_start =
                            always_show_after.take().unwrap_or(std::usize::MAX);
                        always_show_after =
                            Some(cmp::min(start, previous_start));
                    }
                    (None, Some(end)) => {
                        for step in 1..end {
                            steps.insert(step);
                        }

                        last_known_step = cmp::max(last_known_step, end);
                    }
                    _ => {}
                }
            }

            Some(AnimatedElement {
                element,
                class,
                steps,
                always_show_after,
                last_known_step,
            })
        } else {
            None
        }
    }

    fn step_ranges<'a>(
        step_spec: &'a str,
    ) -> impl Iterator<Item = (Option<usize>, Option<usize>)> + 'a {
        step_spec.split(',').map(|range| {
            if range == "-" {
                (Some(1), None)
            } else if range.starts_with('-') {
                let last = range
                    .split('-')
                    .last()
                    .and_then(|step: &str| step.parse().ok());

                if last.is_some() {
                    (Some(1), last)
                } else {
                    (None, None)
                }
            } else if range.ends_with('-') {
                let first = range
                    .split('-')
                    .next()
                    .and_then(|step: &str| step.parse().ok());

                if first.is_some() {
                    (first, None)
                } else {
                    (None, None)
                }
            } else if range.contains('-') {
                let mut steps = range.split('-');
                let first =
                    steps.next().and_then(|step: &str| step.parse().ok());
                let last =
                    steps.last().and_then(|step: &str| step.parse().ok());

                if first.is_some() && last.is_some() {
                    (first, last)
                } else {
                    (None, None)
                }
            } else {
                let step = range.parse().ok();

                (step, step)
            }
        })
    }

    pub fn animate_for_step(&mut self, step: usize) {
        let extra_class = if self.is_shown_in_step(step) {
            "active-in-slide-step"
        } else {
            "inactive-in-slide-step"
        };

        let class = self
            .class
            .as_ref()
            .map(|class| format!("{} {}", class, extra_class))
            .unwrap_or_else(|| extra_class.to_owned());

        let _ = self.element.set_attribute("class", &class);
    }

    pub fn is_shown_in_step(&mut self, step: usize) -> bool {
        if let Some(always_show_after) = self.always_show_after {
            step >= always_show_after || self.steps.contains(&step)
        } else {
            self.steps.contains(&step)
        }
    }

    pub fn last_known_step(&self) -> usize {
        self.last_known_step
    }
}
