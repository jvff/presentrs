use {
    super::{slide::Slide, slide_size::SlideSize},
    lru::LruCache,
    yew::{
        format::{Nothing, Text},
        prelude::*,
        services::fetch::{FetchService, FetchTask, Request, Response},
        virtual_dom::VNode,
    },
};

pub struct Slides {
    locale_path: String,
    slide_cache: LruCache<usize, Status>,
    current_slide: usize,
    current_step: usize,
    link: ComponentLink<Slides>,
    size: SlideSize,
    on_slide_loaded: Option<Callback<usize>>,
}

impl Slides {
    fn fetch_slides(&mut self) {
        for offset in -1..3 {
            let index = self.current_slide as isize + offset;

            if index >= 0 {
                self.fetch_slide(index as usize);
            }
        }
    }

    fn fetch_slide(&mut self, slide_index: usize) {
        match self.slide_cache.peek(&slide_index) {
            None | Some(Status::Error { .. }) => {}
            Some(Status::Loading(_)) | Some(Status::Ready(_)) => return,
        }

        let request = Request::get(format!(
            "/slides/{}{}.html",
            self.locale_path, slide_index
        ))
        .body(Nothing);

        match request {
            Ok(request) => {
                let fetch_task = FetchService::fetch(
                    request,
                    self.link.callback(move |response: Response<Text>| {
                        let (meta, body) = response.into_parts();

                        if meta.status.is_success() {
                            match body {
                                Ok(contents) => Message::LoadComplete(
                                    slide_index,
                                    Ok(contents),
                                ),
                                Err(error) => Message::LoadComplete(
                                    slide_index,
                                    Err(error.to_string()),
                                ),
                            }
                        } else {
                            Message::LoadComplete(
                                slide_index,
                                Err(format!("Get error: {}", meta.status)),
                            )
                        }
                    }),
                )
                .expect("Failed to fetch slide");

                self.slide_cache
                    .put(slide_index, Status::Loading(fetch_task));
            }
            Err(error) => {
                self.slide_cache.put(
                    slide_index,
                    Status::Error {
                        description: "Failed to download slide",
                        cause: Some(error.to_string()),
                        contents: None,
                    },
                );
            }
        }
    }

    fn animate_slide(&mut self) {
        let current_slide = self.slide_cache.get_mut(&self.current_slide);

        if let Some(Status::Ready(ref mut slide)) = current_slide {
            slide.animate_for_step(self.current_step);
        }
    }
}

impl Component for Slides {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let locale_path = properties
            .locale
            .map(|mut locale| {
                locale.push('/');
                locale
            })
            .unwrap_or_else(String::new);

        let mut this = Slides {
            locale_path,
            slide_cache: LruCache::new(10),
            current_slide: properties.current_slide,
            current_step: properties.current_step,
            link,
            size: properties.size,
            on_slide_loaded: properties.on_slide_loaded,
        };

        this.fetch_slides();
        this
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::LoadComplete(index, Ok(contents)) => {
                match Slide::from_html(contents.trim()) {
                    Ok(slide) => {
                        let num_steps = slide.num_steps();

                        self.slide_cache.put(index, Status::Ready(slide));

                        if index == self.current_slide {
                            self.animate_slide();

                            if let Some(ref callback) = self.on_slide_loaded {
                                callback.emit(num_steps);
                            }
                        }
                    }
                    Err(error) => {
                        self.slide_cache.put(
                            index,
                            Status::Error {
                                description: "Slide is not valid HTML",
                                cause: Some(error),
                                contents: Some(contents),
                            },
                        );
                    }
                }
            }
            Message::LoadComplete(index, Err(error)) => {
                self.slide_cache.put(
                    index,
                    Status::Error {
                        description: "Failed to download slide",
                        cause: Some(error),
                        contents: None,
                    },
                );
            }
        };

        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.size = properties.size;

        if self.current_slide != properties.current_slide {
            self.current_slide = properties.current_slide;
            self.fetch_slides();
        }

        if self.current_step != properties.current_step {
            self.current_step = properties.current_step;
            self.animate_slide();
        }

        true
    }

    fn view(&self) -> Html {
        let slide_status = self.slide_cache.peek(&self.current_slide);

        html! {
            <div
                id={"slide"}
                style={ self.size.to_string() }
                class={ format!("current-slide-step-{}", self.current_step) }
            >
                {
                    match slide_status {
                        Some(Status::Loading(_)) => html! {
                            <p>{"Loading slide"}</p>
                        },
                        Some(Status::Ready(ref slide)) => {
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
                        Some(Status::Error {
                            description,
                            ref cause,
                            ref contents,
                        }) => {
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
                        None => {
                            html! {
                                <div>
                                    <p><strong>{"Fatal error"}</strong></p>
                                    <p>{"Slide was not loaded"}</p>
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

#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    #[prop_or_default]
    pub locale: Option<String>,
    #[prop_or(1)]
    pub current_slide: usize,
    #[prop_or(1)]
    pub current_step: usize,
    #[prop_or_default]
    pub size: SlideSize,
    #[prop_or_default]
    pub on_slide_loaded: Option<Callback<usize>>,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            locale: None,
            current_slide: 1,
            current_step: 1,
            size: SlideSize::default(),
            on_slide_loaded: None,
        }
    }
}

pub enum Message {
    LoadComplete(usize, Result<String, String>),
}
