mod navigation;
mod navigation_button;
mod notes;
mod slide;
mod slide_size;
mod slide_sync;
mod slides;

use {
    self::{
        navigation::Navigation, notes::Notes, slide_size::SlideSize,
        slides::Slides,
    },
    lru::LruCache,
    yew::{
        prelude::*,
        services::resize::{ResizeService, ResizeTask, WindowDimensions},
    },
};

const SLIDE_WIDTH: f64 = 800.0;
const SLIDE_HEIGHT: f64 = 600.0;

pub enum Message {
    ToggleNotes,
    SlideLoaded(usize, usize),
    FirstSlide,
    PreviousSlide,
    PreviousStep,
    NextSlide,
    NextStep,
    Resize(WindowDimensions),
    TogglePresent,
    ChangePosition { slide: u16, step: u16 },
    Ignore,
}

pub struct Presentrs {
    component_link: ComponentLink<Self>,
    locale: Option<String>,
    locales: Vec<String>,
    current_slide: usize,
    current_step: usize,
    slide_steps_cache: LruCache<usize, usize>,
    slide_size: SlideSize,
    show_notes: bool,
    presenting: bool,
    _resize_listener: ResizeTask,
}

impl Presentrs {
    fn resize(&mut self, dimensions: WindowDimensions) {
        self.slide_size.resize_to_fit_in(
            dimensions.width as f64,
            dimensions.height as f64,
        );
    }

    fn on_key_down(event: KeyboardEvent) -> Message {
        let message = match event.key().as_str() {
            "ArrowLeft" | "PageUp" => Message::PreviousStep,
            "ArrowRight" | "PageDown" => Message::NextStep,
            "ArrowUp" => Message::PreviousSlide,
            "ArrowDown" => Message::NextSlide,
            "Home" => Message::FirstSlide,
            "n" => Message::ToggleNotes,
            "p" => Message::TogglePresent,
            _ => return Message::Ignore,
        };

        event.prevent_default();

        message
    }
}

impl Component for Presentrs {
    type Message = Message;
    type Properties = Properties;

    fn create(
        properties: Self::Properties,
        component_link: ComponentLink<Self>,
    ) -> Self {
        let window = web_sys::window().expect("Failed to access window");
        let window_size = WindowDimensions::get_dimensions(&window);
        let resize_callback = component_link.callback(Message::Resize);

        let mut slide_size = SlideSize::new(SLIDE_WIDTH, SLIDE_HEIGHT);

        slide_size.resize_to_fit_in(
            window_size.width as f64,
            window_size.height as f64,
        );

        Presentrs {
            component_link,
            locale: properties.locales.first().cloned(),
            locales: properties.locales,
            current_slide: 1,
            current_step: 1,
            slide_steps_cache: LruCache::new(50),
            slide_size,
            show_notes: false,
            presenting: false,
            _resize_listener: ResizeService::register(resize_callback),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::ToggleNotes => {
                self.show_notes = !self.show_notes;
            }
            Message::SlideLoaded(slide_index, num_steps) => {
                self.slide_steps_cache.put(slide_index, num_steps.max(1));

                if self.current_slide == slide_index
                    && self.current_step == usize::MAX
                {
                    self.current_step = num_steps;
                }
            }
            Message::FirstSlide => {
                if self.current_slide != 1 {
                    self.current_slide = 1;
                }

                self.current_step = 1;
            }
            Message::PreviousSlide => {
                if self.current_slide > 1 {
                    self.current_slide -= 1;
                }

                self.current_step = 1;
            }
            Message::PreviousStep => {
                if self.current_step > 1 {
                    self.current_step -= 1;
                } else if self.current_slide > 1 {
                    self.current_slide -= 1;
                    self.current_step = self
                        .slide_steps_cache
                        .peek(&self.current_slide)
                        .copied()
                        .unwrap_or(usize::MAX);
                }
            }
            Message::NextStep => {
                let last_step = self
                    .slide_steps_cache
                    .peek(&self.current_slide)
                    .copied()
                    .unwrap_or(usize::MAX);

                if self.current_step < last_step {
                    self.current_step += 1;
                } else {
                    self.current_slide += 1;
                    self.current_step = 1;
                }
            }
            Message::NextSlide => {
                self.current_slide += 1;
                self.current_step = 1;
            }
            Message::ChangePosition { slide, step } => {
                let slide: usize = slide.into();

                if self.current_slide != slide {
                    self.current_slide = slide.into();
                }

                self.current_step = step.into();
            }
            Message::TogglePresent => self.presenting = !self.presenting,
            Message::Resize(dimensions) => self.resize(dimensions),
            Message::Ignore => return false,
        }
        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.locales = properties.locales;

        if self.locale.is_none() && !self.locales.is_empty() {
            self.locale = self.locales.first().cloned();
        } else if let Some(locale) = self.locale.as_ref() {
            if !self.locales.contains(locale) {
                self.locale = self.locales.first().cloned();
            }
        }

        true
    }

    fn view(&self) -> Html {
        let key_down_callback = self.component_link.callback(Self::on_key_down);
        let slide_loaded_callback =
            self.component_link
                .callback(|(slide_index, slide_step_count)| {
                    Message::SlideLoaded(slide_index, slide_step_count)
                });
        let previous_slide_callback =
            self.component_link.callback(|_| Message::PreviousSlide);
        let previous_step_callback =
            self.component_link.callback(|_| Message::PreviousStep);
        let next_slide_callback =
            self.component_link.callback(|_| Message::NextSlide);
        let next_step_callback =
            self.component_link.callback(|_| Message::NextStep);
        let update_position_callback = self
            .component_link
            .callback(|(slide, step)| Message::ChangePosition { slide, step });

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
                    locale = self.locale.clone()
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
                    on_update_position = update_position_callback
                    presenting = self.presenting
                    current_slide = self.current_slide
                    current_step = self.current_step
                    />
            </div>
        }
    }
}

#[derive(Clone, Debug, Default, Properties)]
pub struct Properties {
    locales: Vec<String>,
}

impl Properties {
    pub fn with_locales<I>(mut self, locales: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.locales = locales.into_iter().map(|item| item.into()).collect();
        self
    }

    pub fn set_locales<I>(&mut self, locales: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.locales = locales.into_iter().map(|item| item.into()).collect();
        self
    }
}
