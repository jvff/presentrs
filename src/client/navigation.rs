use yew::prelude::*;

pub struct Navigation {
    on_previous_slide: Option<Callback<()>>,
    on_previous_step: Option<Callback<()>>,
    on_next_step: Option<Callback<()>>,
    on_next_slide: Option<Callback<()>>,
}

impl Component for Navigation {
    type Message = Message;
    type Properties = Properties;

    fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self {
        Navigation {
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
        html! {
            <div style={
                "position: absolute;
                 bottom: 0;
                 width: 97%;"
            }>
                <form onsubmit="return false;" style={
                    "margin-left: auto; margin-right: auto;"
                }>
                    <button type="submit" onclick=|_| Message::PreviousSlide>
                        {"Previous slide"}
                    </button>
                    <button type="submit" onclick=|_| Message::PreviousStep>
                        {"Previous step"}
                    </button>
                    <button type="submit" onclick=|_| Message::NextStep>
                        {"Next step"}
                    </button>
                    <button type="submit" onclick=|_| Message::NextSlide>
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

#[derive(Clone, PartialEq)]
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
