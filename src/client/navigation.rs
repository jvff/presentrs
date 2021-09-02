use {
    super::{
        navigation_button::{Direction, NavigationButton, Target},
        slide_sync::SlideSync,
    },
    yew::prelude::*,
};

pub struct Navigation {
    sync_url: String,
    on_previous_slide: Option<Callback<()>>,
    on_previous_step: Option<Callback<()>>,
    on_next_step: Option<Callback<()>>,
    on_next_slide: Option<Callback<()>>,
    on_update_position: Callback<(u16, u16)>,
    presenting: bool,
    current_slide: usize,
    current_step: usize,
}

impl Component for Navigation {
    type Message = ();
    type Properties = Properties;

    fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self {
        let window = web_sys::window().expect("Failed to access window");
        let host = window.location().host().expect("Invalid host location");

        Navigation {
            sync_url: format!("ws://{}/sync", host),
            on_previous_slide: properties.on_previous_slide,
            on_previous_step: properties.on_previous_step,
            on_next_step: properties.on_next_step,
            on_next_slide: properties.on_next_slide,
            on_update_position: properties.on_update_position,
            presenting: properties.presenting,
            current_slide: properties.current_slide,
            current_step: properties.current_step,
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
        self.on_update_position = properties.on_update_position;
        self.presenting = properties.presenting;
        self.current_slide = properties.current_slide;
        self.current_step = properties.current_step;

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
                    <SlideSync
                        url = self.sync_url.clone()
                        presenting = self.presenting
                        current_slide = self.current_slide
                        current_step = self.current_step
                        on_update_position = &self.on_update_position
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
    pub on_update_position: Callback<(u16, u16)>,
    pub presenting: bool,
    pub current_slide: usize,
    pub current_step: usize,
}
