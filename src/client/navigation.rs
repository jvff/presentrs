use {
    super::navigation_button::{Direction, NavigationButton, Target},
    yew::prelude::*,
};

pub struct Navigation {
    on_previous_slide: Option<Callback<()>>,
    on_previous_step: Option<Callback<()>>,
    on_next_step: Option<Callback<()>>,
    on_next_slide: Option<Callback<()>>,
}

impl Component for Navigation {
    type Message = ();
    type Properties = Properties;

    fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self {
        Navigation {
            on_previous_slide: properties.on_previous_slide,
            on_previous_step: properties.on_previous_step,
            on_next_step: properties.on_next_step,
            on_next_slide: properties.on_next_slide,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
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
            <div style="
                position: absolute;\
                left: 0;\
                right: 0;\
                bottom: 0;\
            ">
                <div style="\
                    margin-left: auto;\
                    margin-right: auto;\
                    display: table;\
                ">
                    <NavigationButton
                        direction=Direction::Backward
                        on_click=&self.on_previous_slide
                        target=Target::Slide
                        />
                    <NavigationButton
                        direction=Direction::Backward
                        on_click=&self.on_previous_step
                        target=Target::Step
                        />
                    <NavigationButton
                        direction=Direction::Forward
                        on_click=&self.on_next_step
                        target=Target::Step
                        />
                    <NavigationButton
                        direction=Direction::Forward
                        on_click=&self.on_next_slide
                        target=Target::Slide
                        />
                </div>
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
