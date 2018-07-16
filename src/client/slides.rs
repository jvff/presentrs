use stdweb::web::Node;
use yew::format::{Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::VNode;

pub struct Slides {
    current_slide: usize,
    current_step: usize,
    status: Status,
    fetch_service: FetchService,
    link: ComponentLink<Slides>,
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
        let mut changed = self.current_step != properties.current_step;

        self.current_step = properties.current_step;

        if self.current_slide != properties.current_slide {
            self.current_slide = properties.current_slide;
            self.fetch_slide();
            changed = true;
        }

        changed
    }
}

impl Renderable<Slides> for Slides {
    fn view(&self) -> Html<Self> {
        html! {
            <div class={ format!("current-slide-step-{}", self.current_step) },>
                {
                    match self.status {
                        Status::Loading(_) => html! {
                            <p>{"Loading slide"}</p>
                        },
                        Status::Ready(ref contents) => {
                            match Node::from_html(contents) {
                                Ok(contents) => VNode::VRef(contents),
                                Err(_) => html! {
                                    <p>{"Slide is not valid HTML"}</p>
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
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            current_slide: 1,
            current_step: 1,
        }
    }
}

pub enum Message {
    LoadComplete(Option<String>),
}
