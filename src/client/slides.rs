use yew::format::{Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::VNode;

use super::slide::Slide;
use super::slide_size::SlideSize;

pub struct Slides {
    current_slide: usize,
    current_step: usize,
    status: Status,
    fetch_service: FetchService,
    link: ComponentLink<Slides>,
    size: SlideSize,
    on_slide_loaded: Option<Callback<usize>>,
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

    fn animate_slide(&mut self) {
        if let Status::Ready(ref mut slide) = self.status {
            slide.animate_for_step(self.current_step);
        }
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
            on_slide_loaded: properties.on_slide_loaded,
        };

        this.fetch_slide();
        this
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::LoadComplete(Ok(contents)) => {
                match Slide::from_html(contents.trim()) {
                    Ok(slide) => {
                        let num_steps = slide.num_steps();

                        self.status = Status::Ready(slide);
                        self.animate_slide();

                        if let Some(ref callback) = self.on_slide_loaded {
                            callback.emit(num_steps);
                        }
                    }
                    Err(error) => {
                        self.status = Status::Error {
                            description: "Slide is not valid HTML",
                            cause: Some(error),
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

        if self.current_slide != properties.current_slide {
            self.current_slide = properties.current_slide;
            self.fetch_slide();
        }

        if self.current_step != properties.current_step {
            self.current_step = properties.current_step;
            self.animate_slide();
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div
                id={"slide"}
                style={ self.size.to_string() }
                class={ format!("current-slide-step-{}", self.current_step) }
            >
                {
                    match self.status {
                        Status::Loading(_) => html! {
                            <p>{"Loading slide"}</p>
                        },
                        Status::Ready(ref slide) => {
                            match slide.as_node() {
                                Ok(node) => VNode::VRef(node),
                                Err(error) => html! {
                                    <div>
                                        <p><strong>
                                            {"Failed to animate slide"}
                                        </strong></p>
                                        <p>{format!("Error: {}", error)}</p>
                                    </div>
                                },
                            }
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
    Ready(Slide),
    Error {
        description: &'static str,
        cause: Option<String>,
        contents: Option<String>,
    },
}

#[derive(Clone, PartialEq)]
pub struct Properties {
    pub current_slide: usize,
    pub current_step: usize,
    pub size: SlideSize,
    pub on_slide_loaded: Option<Callback<usize>>,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            current_slide: 1,
            current_step: 1,
            size: SlideSize::default(),
            on_slide_loaded: None,
        }
    }
}

pub enum Message {
    LoadComplete(Result<String, String>),
}
