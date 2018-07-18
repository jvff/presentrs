use stdweb::web::event::ResizeEvent;
use stdweb::web::{self, IEventTarget, Node};
use yew::format::{Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::VNode;

const SLIDE_WIDTH: f64 = 800.0;
const SLIDE_HEIGHT: f64 = 600.0;

pub struct Slides {
    current_slide: usize,
    current_step: usize,
    status: Status,
    fetch_service: FetchService,
    link: ComponentLink<Slides>,
    scale: f64,
    translate_x: f64,
    translate_y: f64,
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

    fn resize(&mut self) {
        let window = web::window();
        let width = window.inner_width() as f64;
        let height = window.inner_height() as f64;

        let scale_x = width / SLIDE_WIDTH;
        let scale_y = height / SLIDE_HEIGHT;

        self.scale = scale_x.min(scale_y);

        let new_width = SLIDE_WIDTH * self.scale;
        let new_height = SLIDE_HEIGHT * self.scale;

        let delta_x = new_width - SLIDE_WIDTH;
        let delta_y = new_height - SLIDE_HEIGHT;

        self.translate_x = delta_x / 2.0;
        self.translate_y = delta_y / 2.0;
    }
}

impl Component for Slides {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let window = web::window();
        let window_resize_callback = link.send_back(|_| Message::Resize);

        window.add_event_listener(move |_: ResizeEvent| {
            window_resize_callback.emit(());
        });

        let mut this = Slides {
            current_slide: properties.current_slide,
            current_step: properties.current_step,
            status: Status::Error,
            fetch_service: FetchService::new(),
            link,
            scale: 1.0,
            translate_x: 0.0,
            translate_y: 0.0,
        };

        this.fetch_slide();
        this.resize();
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
            Message::Resize => {
                self.resize();
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
            <div
                id={"slide-container"},
                style={ format!(
                    "width: {}px; height: {}px;
                     transform: translate({}px, {}px) scale({})",
                     SLIDE_WIDTH, SLIDE_HEIGHT,
                     self.translate_x, self.translate_y, self.scale,
                )},
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
    Resize,
}
