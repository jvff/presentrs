use yew::format::{Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::VNode;

pub struct Notes {
    current_slide: usize,
    current_step: usize,
    enabled: bool,
    status: Status,
}

impl Component for Notes {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let get_notes_request = Request::get("/notes.html").body(Nothing);
        let status;

        if let Ok(request) = get_notes_request {
            let fetch_task = FetchService::fetch(
                request,
                link.callback(|response: Response<Text>| {
                    let (meta, body) = response.into_parts();

                    if meta.status.is_success() {
                        if let Ok(notes) = body {
                            Message::LoadComplete(Some(notes))
                        } else {
                            Message::LoadComplete(None)
                        }
                    } else {
                        Message::LoadComplete(None)
                    }
                }),
            )
            .expect("Failed to fetch notes");

            status = Status::Loading(fetch_task);
        } else {
            status = Status::Error;
        }

        Notes {
            current_slide: properties.current_slide,
            current_step: properties.current_step,
            enabled: properties.enabled,
            status,
        }
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
        self.current_slide = properties.current_slide;
        self.current_step = properties.current_step;
        self.enabled = properties.enabled;
        true
    }

    fn view(&self) -> Html {
        if self.enabled {
            html! {
                <div class={
                    format!(
                        "current-slide-{} current-slide-step-{}",
                        self.current_slide, self.current_step
                    )
                }>
                    {
                        match self.status {
                            Status::Loading(_) => html! {
                                <p>{"Loading notes"}</p>
                            },
                            Status::Ready(ref notes) => {
                                let window = web_sys::window().expect("Failed to access window");
                                let document = window.document().expect("Window has no document");
                                let parent_element = document
                                    .create_element("div")
                                    .expect("Failed to create <div> element");

                                parent_element.set_inner_html(notes.trim());

                                let element = parent_element
                                    .first_element_child()
                                    .expect("Invalid HTML for notes");

                                VNode::VRef(element.into())
                            }
                            Status::Error => html! {
                                <p>{"Failed to load notes"}</p>
                            },
                        }
                    }
                </div>
            }
        } else {
            html!(<div style={"display: none"}></div>)
        }
    }
}

pub enum Status {
    Loading(FetchTask),
    Ready(String),
    Error,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Properties {
    #[prop_or(1)]
    pub current_slide: usize,
    #[prop_or(1)]
    pub current_step: usize,
    #[prop_or(false)]
    pub enabled: bool,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            current_slide: 1,
            current_step: 1,
            enabled: false,
        }
    }
}

pub enum Message {
    LoadComplete(Option<String>),
}
