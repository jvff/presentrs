use std::cmp;
use std::collections::HashSet;

use stdweb::unstable::TryFrom as StdWebTryFrom;
use stdweb::web::{CloneKind, Element, IElement, INode, Node};

pub struct Slide {
    contents: Node,
    num_steps: usize,
    animated_elements: Vec<AnimatedElement>,
}

impl Slide {
    pub fn from_html(html: &str) -> Result<Self, String> {
        match Node::from_html(html) {
            Ok(contents) => Ok(Self::new(contents)),
            Err(error) => Err(error.to_string()),
        }
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
        let maybe_element: Result<Element, _> =
            StdWebTryFrom::try_from(node.clone());

        if let Ok(element) = maybe_element {
            if let Some(animated_element) = AnimatedElement::try_from(element) {
                *max_steps =
                    cmp::max(*max_steps, animated_element.last_known_step());

                animated_elements.push(animated_element);
            }
        }

        for child in node.child_nodes().iter() {
            Self::load_contents(&child, max_steps, animated_elements);
        }
    }

    pub fn animate_for_step(&mut self, step: usize) {
        for animated_element in self.animated_elements.iter_mut() {
            animated_element.animate_for_step(step);
        }
    }

    pub fn as_node(&self) -> Result<Node, String> {
        self.contents
            .clone_node(CloneKind::Deep)
            .map_err(|_| "Failed to clone node tree".to_owned())
    }
}

#[derive(Clone, Debug)]
struct AnimatedElement {
    element: Element,
    class: Option<String>,
    steps: HashSet<usize>,
    shown_till_end: bool,
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
            let mut last_known_step = 0;
            let mut shown_till_end = false;

            for range in Self::step_ranges(step_spec.trim()) {
                match range {
                    (Some(start), Some(end)) => {
                        for step in start..end {
                            steps.insert(step);
                        }

                        last_known_step = cmp::max(last_known_step, end);
                    }
                    (Some(start), None) => {
                        for step in start..last_known_step {
                            steps.insert(step);
                        }

                        last_known_step = cmp::max(last_known_step, start);
                        shown_till_end = true;
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
                shown_till_end,
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
        self.steps.contains(&step)
            || (step >= self.last_known_step && self.shown_till_end)
    }

    pub fn last_known_step(&self) -> usize {
        self.last_known_step
    }
}
