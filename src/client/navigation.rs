use yew::prelude::*;

pub struct Navigation {
    component_link: ComponentLink<Self>,
    on_previous_slide: Option<Callback<()>>,
    on_previous_step: Option<Callback<()>>,
    on_next_step: Option<Callback<()>>,
    on_next_slide: Option<Callback<()>>,
}

impl Component for Navigation {
    type Message = Message;
    type Properties = Properties;

    fn create(
        properties: Self::Properties,
        component_link: ComponentLink<Self>,
    ) -> Self {
        Navigation {
            component_link,
            on_previous_slide: properties.on_previous_slide,
            on_previous_step: properties.on_previous_step,
            on_next_step: properties.on_next_step,
            on_next_slide: properties.on_next_slide,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::PreviousSlide => {
                if let Some(ref callback) = self.on_previous_slide {
                    callback.emit(());
                }
            }
            Message::PreviousStep => {
                if let Some(ref callback) = self.on_previous_step {
                    callback.emit(());
                }
            }
            Message::NextStep => {
                if let Some(ref callback) = self.on_next_step {
                    callback.emit(());
                }
            }
            Message::NextSlide => {
                if let Some(ref callback) = self.on_next_slide {
                    callback.emit(());
                }
            }
        }
        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.on_previous_slide = properties.on_previous_slide;
        self.on_previous_step = properties.on_previous_step;
        self.on_next_step = properties.on_next_step;
        self.on_next_slide = properties.on_next_slide;

        true
    }

    fn view(&self) -> Html {
        let previous_slide_callback =
            self.component_link.callback(|_| Message::PreviousSlide);
        let previous_step_callback =
            self.component_link.callback(|_| Message::PreviousStep);
        let next_slide_callback =
            self.component_link.callback(|_| Message::NextSlide);
        let next_step_callback =
            self.component_link.callback(|_| Message::NextStep);

        html! {
            <div style={
                "position: absolute;
                 bottom: 0;
                 width: 97%;"
            }>
                <form style={
                    "margin-left: auto; margin-right: auto;"
                }>
                    <button onclick=previous_slide_callback>
                        {"Previous slide"}
                    </button>
                    <button onclick=previous_step_callback>
                        {"Previous step"}
                    </button>
                    <button onclick=next_step_callback>
                        {"Next step"}
                    </button>
                    <button onclick=next_slide_callback>
                        {"Next slide"}
                    </button>
                </form>
            </div>
        }
    }
}

pub enum Message {
    PreviousSlide,
    PreviousStep,
    NextSlide,
    NextStep,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub on_previous_slide: Option<Callback<()>>,
    pub on_previous_step: Option<Callback<()>>,
    pub on_next_step: Option<Callback<()>>,
    pub on_next_slide: Option<Callback<()>>,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            on_previous_slide: None,
            on_previous_step: None,
            on_next_step: None,
            on_next_slide: None,
        }
    }
}
