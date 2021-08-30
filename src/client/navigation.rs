use yew::prelude::*;

pub struct Navigation {
    on_previous_slide: Option<Callback<MouseEvent>>,
    on_previous_step: Option<Callback<MouseEvent>>,
    on_next_step: Option<Callback<MouseEvent>>,
    on_next_slide: Option<Callback<MouseEvent>>,
}

impl Navigation {
    fn build_callback(
        callback: Option<Callback<()>>,
    ) -> Option<Callback<MouseEvent>> {
        callback.map(|callback| callback.reform(|_| ()))
    }
}

impl Component for Navigation {
    type Message = ();
    type Properties = Properties;

    fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self {
        Navigation {
            on_previous_slide: Self::build_callback(
                properties.on_previous_slide,
            ),
            on_previous_step: Self::build_callback(properties.on_previous_step),
            on_next_step: Self::build_callback(properties.on_next_step),
            on_next_slide: Self::build_callback(properties.on_next_slide),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.on_previous_slide =
            Self::build_callback(properties.on_previous_slide);
        self.on_previous_step =
            Self::build_callback(properties.on_previous_step);
        self.on_next_step = Self::build_callback(properties.on_next_step);
        self.on_next_slide = Self::build_callback(properties.on_next_slide);

        true
    }

    fn view(&self) -> Html {
        html! {
            <div style={
                "position: absolute;
                 bottom: 0;
                 width: 97%;"
            }>
                <form style={
                    "margin-left: auto; margin-right: auto;"
                }>
                    <button onclick=&self.on_previous_slide>
                        {"Previous slide"}
                    </button>
                    <button onclick=&self.on_previous_step>
                        {"Previous step"}
                    </button>
                    <button onclick=&self.on_next_step>
                        {"Next step"}
                    </button>
                    <button onclick=&self.on_next_slide>
                        {"Next slide"}
                    </button>
                </form>
            </div>
        }
    }
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
