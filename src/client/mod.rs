mod navigation;
mod notes;
mod slide;
mod slide_size;
mod slides;

use std;

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Event;

use yew::prelude::*;

use self::navigation::Navigation;
use self::notes::Notes;
use self::slide_size::SlideSize;
use self::slides::Slides;

const SLIDE_WIDTH: f64 = 800.0;
const SLIDE_HEIGHT: f64 = 600.0;

pub enum Message {
    ToggleNotes,
    SlideLoaded(usize),
    FirstSlide,
    PreviousSlide,
    PreviousStep,
    NextSlide,
    NextStep,
    Resize,
    Ignore,
}

pub struct Presentrs {
    component_link: ComponentLink<Self>,
    current_slide: usize,
    current_step: usize,
    current_slide_steps: Option<usize>,
    slide_size: SlideSize,
    show_notes: bool,
}

impl Presentrs {
    fn resize(&mut self) {
        let window = web_sys::window().expect("Failed to access window");
        let width = window
            .inner_width()
            .expect("Failed to get inner window width")
            .as_f64()
            .expect("Inner window width is not a number");
        let height = window
            .inner_height()
            .expect("Failed to get inner window height")
            .as_f64()
            .expect("Inner window height is not a number");

        self.slide_size.resize_to_fit_in(width, height);
    }

    fn on_key_down(event: KeyboardEvent) -> Message {
        let message = match event.key().as_str() {
            "ArrowLeft" | "PageUp" => Message::PreviousStep,
            "ArrowRight" | "PageDown" => Message::NextStep,
            "ArrowUp" => Message::PreviousSlide,
            "ArrowDown" => Message::NextSlide,
            "Home" => Message::FirstSlide,
            "n" => Message::ToggleNotes,
            _ => return Message::Ignore,
        };

        event.prevent_default();

        message
    }
}

impl Component for Presentrs {
    type Message = Message;
    type Properties = ();

    fn create(
        _: Self::Properties,
        component_link: ComponentLink<Self>,
    ) -> Self {
        let window = web_sys::window().expect("Failed to access window");
        let component_resize_callback =
            component_link.callback(|_| Message::Resize);
        let window_resize_callback = Closure::wrap(Box::new(move |_: &Event| {
            component_resize_callback.emit(())
        })
            as Box<dyn Fn(&Event)>);

        window.set_onresize(Some(
            window_resize_callback.as_ref().unchecked_ref(),
        ));

        let mut this = Presentrs {
            component_link,
            current_slide: 1,
            current_step: 1,
            current_slide_steps: None,
            slide_size: SlideSize::new(SLIDE_WIDTH, SLIDE_HEIGHT),
            show_notes: false,
        };

        this.resize();
        this
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::ToggleNotes => {
                self.show_notes = !self.show_notes;
            }
            Message::SlideLoaded(num_steps) => {
                self.current_slide_steps = Some(num_steps.max(1));

                if self.current_step == std::usize::MAX {
                    self.current_step = num_steps;
                }
            }
            Message::FirstSlide => {
                self.current_slide = 1;
                self.current_step = 1;
                self.current_slide_steps = None;
            }
            Message::PreviousSlide => {
                if self.current_slide > 1 {
                    self.current_slide -= 1;
                }
                self.current_step = 1;
                self.current_slide_steps = None;
            }
            Message::PreviousStep => {
                if self.current_step > 1 && self.current_slide_steps.is_some() {
                    self.current_step -= 1;
                } else if self.current_slide > 1 {
                    self.current_slide -= 1;
                    self.current_step = std::usize::MAX;
                    self.current_slide_steps = None;
                }
            }
            Message::NextStep => {
                let last_step =
                    self.current_slide_steps.unwrap_or(std::usize::MAX);

                if self.current_step < last_step {
                    self.current_step += 1;
                } else {
                    self.current_slide += 1;
                    self.current_step = 1;
                    self.current_slide_steps = None;
                }
            }
            Message::NextSlide => {
                self.current_slide += 1;
                self.current_step = 1;
                self.current_slide_steps = None;
            }
            Message::Resize => self.resize(),
            Message::Ignore => return false,
        }
        true
    }

    fn change(&mut self, _properties: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let key_down_callback = self.component_link.callback(Self::on_key_down);
        let slide_loaded_callback =
            self.component_link.callback(Message::SlideLoaded);
        let previous_slide_callback =
            self.component_link.callback(|_| Message::PreviousSlide);
        let previous_step_callback =
            self.component_link.callback(|_| Message::PreviousStep);
        let next_slide_callback =
            self.component_link.callback(|_| Message::NextSlide);
        let next_step_callback =
            self.component_link.callback(|_| Message::NextStep);

        html! {
            <div
                tabindex = 0
                onkeydown = key_down_callback
                style = {"
                    position: absolute;
                    left: 0;
                    right: 0;
                    top: 0;
                    bottom: 0;
                "}
                >
                <Slides
                    current_slide = self.current_slide
                    current_step = self.current_step
                    size = self.slide_size
                    on_slide_loaded = slide_loaded_callback
                    />
                <Notes
                    current_slide = self.current_slide
                    current_step = self.current_step
                    enabled = self.show_notes
                    />
                <Navigation
                    on_previous_slide = previous_slide_callback
                    on_previous_step = previous_step_callback
                    on_next_step = next_step_callback
                    on_next_slide = next_slide_callback
                    />
            </div>
        }
    }
}
