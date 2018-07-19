use stdweb::unstable::TryFrom as StdWebTryFrom;
use stdweb::web::{CloneKind, Element, IElement, INode, Node};

use yew::format::{Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::VNode;

use super::slide_size::SlideSize;

pub struct Slides {
    current_slide: usize,
    current_step: usize,
    status: Status,
    fetch_service: FetchService,
    link: ComponentLink<Slides>,
    size: SlideSize,
}

impl Slides {
    fn fetch_slide(&mut self) {
        let request =
            Request::get(format!("/slides/{}.html", self.current_slide))
                .body(Nothing);

        match request {
            Ok(request) => {
                let fetch_task = self.fetch_service.fetch(
                    request,
                    self.link.send_back(|response: Response<Text>| {
                        let (meta, body) = response.into_parts();

                        if meta.status.is_success() {
                            match body {
                                Ok(contents) => {
                                    Message::LoadComplete(Ok(contents))
                                }
                                Err(error) => Message::LoadComplete(Err(
                                    error.to_string()
                                )),
                            }
                        } else {
                            Message::LoadComplete(Err(format!(
                                "Get error: {}",
                                meta.status
                            )))
                        }
                    }),
                );

                self.status = Status::Loading(fetch_task);
            }
            Err(error) => {
                self.status = Status::Error {
                    description: "Failed to download slide",
                    cause: Some(error.to_string()),
                    contents: None,
                };
            }
        }
    }

    fn animated_slide(&self, slide: &Node) -> Node {
        let slide = slide.clone_node(CloneKind::Deep).unwrap_or_else(|error| {
            Node::from_html(&format!(
                "<div>
                    <p><strong>Internal error</strong></p>
                    <p>Failed to animate slide</p>
                    <p>Error: {}</p>
                </div>",
                error
            )).expect("Failed to create error HTML fragment")
        });

        self.animate_element(slide.clone());

        slide
    }

    fn animate_element(&self, node: Node) {
        let maybe_element: Result<Element, _> =
            StdWebTryFrom::try_from(node.clone());

        if let Ok(element) = maybe_element {
            if let Some(steps) = element.get_attribute("data-slide-steps") {
                let mut class = element
                    .get_attribute("class")
                    .map(|mut class| {
                        if !class.ends_with(' ') {
                            class.push(' ')
                        }
                        class
                    })
                    .unwrap_or_else(|| String::new());

                if self.includes_current_step(steps.trim()) {
                    class.push_str("active-in-slide-step");
                } else {
                    class.push_str("inactive-in-slide-step");
                }

                let _ = element.set_attribute("class", &class);
            }
        }

        for child in node.child_nodes().iter() {
            self.animate_element(child);
        }
    }

    fn includes_current_step(&self, step_spec: &str) -> bool {
        for range in step_spec.split(',') {
            let (maybe_first, maybe_last): (
                Option<usize>,
                Option<usize>,
            ) = if range == "-" {
                (Some(1), Some(::std::usize::MAX))
            } else if range.starts_with('-') {
                let last = range
                    .split('-')
                    .last()
                    .and_then(|step: &str| step.parse().ok());

                (Some(1), last)
            } else if range.ends_with('-') {
                let first = range
                    .split('-')
                    .next()
                    .and_then(|step: &str| step.parse().ok());

                (first, Some(::std::usize::MAX))
            } else if range.contains('-') {
                let mut steps = range.split('-');
                let first =
                    steps.next().and_then(|step: &str| step.parse().ok());
                let last =
                    steps.last().and_then(|step: &str| step.parse().ok());

                (first, last)
            } else {
                let step = range.parse().ok();

                (step, step)
            };

            if let (Some(first), Some(last)) = (maybe_first, maybe_last) {
                if first <= self.current_step && last >= self.current_step {
                    return true;
                }
            }
        }

        false
    }
}

impl Component for Slides {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let status = Status::Error {
            description: "Starting",
            cause: None,
            contents: None,
        };

        let mut this = Slides {
            current_slide: properties.current_slide,
            current_step: properties.current_step,
            status,
            fetch_service: FetchService::new(),
            link,
            size: properties.size,
        };

        this.fetch_slide();
        this
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::LoadComplete(Ok(contents)) => {
                match Node::from_html(contents.trim()) {
                    Ok(node) => self.status = Status::Ready(node),
                    Err(error) => {
                        self.status = Status::Error {
                            description: "Slide is not valid HTML",
                            cause: Some(error.to_string()),
                            contents: Some(contents),
                        };
                    }
                }
            }
            Message::LoadComplete(Err(error)) => {
                self.status = Status::Error {
                    description: "Failed to download slide",
                    cause: Some(error),
                    contents: None,
                };
            }
        }
        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.size = properties.size;
        self.current_step = properties.current_step;

        if self.current_slide != properties.current_slide {
            self.current_slide = properties.current_slide;
            self.fetch_slide();
        }

        true
    }
}

impl Renderable<Slides> for Slides {
    fn view(&self) -> Html<Self> {
        html! {
            <div
                id={"slide"},
                style={ self.size.to_string() },
                class={ format!("current-slide-step-{}", self.current_step) },
            >
                {
                    match self.status {
                        Status::Loading(_) => html! {
                            <p>{"Loading slide"}</p>
                        },
                        Status::Ready(ref contents) => {
                            VNode::VRef(self.animated_slide(contents))
                        }
                        Status::Error {
                            description,
                            ref cause,
                            ref contents,
                        } => {
                            html! {
                                <div>
                                    <p><strong>{description}</strong></p>
                                    {
                                        if let Some(cause) = cause {
                                            html! {
                                                <p>{
                                                    format!("Error: {}", cause)
                                                }</p>
                                            }
                                        } else {
                                            html!{}
                                        }
                                    }
                                    {
                                        if let Some(contents) = contents {
                                            html! {
                                                <p>{ format!(
                                                    "Contents: {}",
                                                    contents,
                                                )}</p>
                                            }
                                        } else {
                                            html!{}
                                        }
                                    }
                                </div>
                            }
                        }
                    }
                }
            </div>
        }
    }
}

pub enum Status {
    Loading(FetchTask),
    Ready(Node),
    Error {
        description: &'static str,
        cause: Option<String>,
        contents: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Properties {
    pub current_slide: usize,
    pub current_step: usize,
    pub size: SlideSize,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            current_slide: 1,
            current_step: 1,
            size: SlideSize::default(),
        }
    }
}

pub enum Message {
    LoadComplete(Result<String, String>),
}
