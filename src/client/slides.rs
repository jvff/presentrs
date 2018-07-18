use stdweb::web::Node;

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

        if let Ok(request) = request {
            let fetch_task = self.fetch_service.fetch(
                request,
                self.link.send_back(|response: Response<Text>| {
                    let (meta, body) = response.into_parts();

                    if meta.status.is_success() {
                        if let Ok(contents) = body {
                            Message::LoadComplete(Some(contents))
                        } else {
                            Message::LoadComplete(None)
                        }
                    } else {
                        Message::LoadComplete(None)
                    }
                }),
            );

            self.status = Status::Loading(fetch_task);
        } else {
            self.status = Status::Error;
        }
    }
}

impl Component for Slides {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut this = Slides {
            current_slide: properties.current_slide,
            current_step: properties.current_step,
            status: Status::Error,
            fetch_service: FetchService::new(),
            link,
            size: properties.size,
        };

        this.fetch_slide();
        this
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::LoadComplete(None) => {
                self.status = Status::Error;
            }
            Message::LoadComplete(Some(notes)) => {
                self.status = Status::Ready(notes);
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
                id={"slide-container"},
                style={ self.size.to_string() },
                class={ format!("current-slide-step-{}", self.current_step) },
            >
                {
                    match self.status {
                        Status::Loading(_) => html! {
                            <p>{"Loading slide"}</p>
                        },
                        Status::Ready(ref contents) => {
                            match Node::from_html(contents.trim()) {
                                Ok(contents) => VNode::VRef(contents),
                                Err(error) => html! {
                                    <p><strong>
                                        {"Slide is not valid HTML"}
                                    </strong></p>
                                    <p>{format!("Error: {}", error)}</p>
                                    <p>{format!("Contents: {}", contents)}</p>
                                },
                            }
                        }
                        Status::Error => html! {
                            <p>{"Failed to load slide"}</p>
                        },
                    }
                }
            </div>
        }
    }
}

pub enum Status {
    Loading(FetchTask),
    Ready(String),
    Error,
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
    LoadComplete(Option<String>),
}
