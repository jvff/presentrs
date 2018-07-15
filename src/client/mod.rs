use stdweb::web::Node;
use yew::format::{Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::VNode;

enum NotesStatus {
    Loading(FetchTask),
    Ready(String),
    LoadFailed,
}

pub enum Message {
    NotesLoaded(Option<String>),
}

pub struct Presentrs {
    notes_status: NotesStatus,
}

impl Component for Presentrs {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut fetch_service = FetchService::new();
        let get_notes_request = Request::get("/notes.html").body(Nothing);
        let notes_status;

        if let Ok(request) = get_notes_request {
            let fetch_task = fetch_service.fetch(
                request,
                link.send_back(|response: Response<Text>| {
                    let (meta, body) = response.into_parts();

                    if meta.status.is_success() {
                        if let Ok(notes) = body {
                            Message::NotesLoaded(Some(notes))
                        } else {
                            Message::NotesLoaded(None)
                        }
                    } else {
                        Message::NotesLoaded(None)
                    }
                }),
            );

            notes_status = NotesStatus::Loading(fetch_task);
        } else {
            notes_status = NotesStatus::LoadFailed;
        }

        Presentrs { notes_status }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::NotesLoaded(None) => {
                self.notes_status = NotesStatus::LoadFailed
            }
            Message::NotesLoaded(Some(notes)) => {
                self.notes_status = NotesStatus::Ready(notes)
            }
        }
        true
    }
}

impl Renderable<Presentrs> for Presentrs {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                {
                    match self.notes_status {
                        NotesStatus::Loading(_) => html! {
                            <p>{"Loading notes"}</p>
                        },
                        NotesStatus::Ready(ref notes) => {
                            match Node::from_html(notes) {
                                Ok(notes) => VNode::VRef(notes),
                                Err(_) => html! {
                                    <p>{"Notes are not valid HTML"}</p>
                                },
                            }
                        }
                        NotesStatus::LoadFailed => html! {
                            <p>{"Failed to load notes"}</p>
                        },
                    }
                }
            </div>
        }
    }
}
