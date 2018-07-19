mod navigation;
mod notes;
mod slide_size;
mod slides;

use stdweb::traits::IEvent;
use stdweb::web::event::ResizeEvent;
use stdweb::web::{self, IEventTarget};

use yew::prelude::*;

use self::navigation::Navigation;
use self::notes::Notes;
use self::slide_size::SlideSize;
use self::slides::Slides;

const SLIDE_WIDTH: f64 = 800.0;
const SLIDE_HEIGHT: f64 = 600.0;

pub enum Message {
    FirstSlide,
    PreviousSlide,
    PreviousStep,
    NextSlide,
    NextStep,
    Resize,
    Ignore,
}

pub struct Presentrs {
    current_slide: usize,
    current_step: usize,
    slide_size: SlideSize,
}

impl Presentrs {
    fn resize(&mut self) {
        let window = web::window();
        let width = window.inner_width() as f64;
        let height = window.inner_height() as f64;

        self.slide_size.resize_to_fit_in(width, height);
    }

    fn on_key_down(event: KeyDownEvent) -> Message {
        let message = match event.key().as_str() {
            "ArrowLeft" | "PageUp" => Message::PreviousStep,
            "ArrowRight" | "PageDown" => Message::NextStep,
            "ArrowUp" => Message::PreviousSlide,
            "ArrowDown" => Message::NextSlide,
            "Home" => Message::FirstSlide,
            _ => return Message::Ignore,
        };

        event.prevent_default();

        message
    }
}

impl Component for Presentrs {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let window = web::window();
        let window_resize_callback = link.send_back(|_| Message::Resize);

        window.add_event_listener(move |_: ResizeEvent| {
            window_resize_callback.emit(());
        });

        let mut this = Presentrs {
            current_slide: 1,
            current_step: 1,
            slide_size: SlideSize::new(SLIDE_WIDTH, SLIDE_HEIGHT),
        };

        this.resize();
        this
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::FirstSlide => {
                self.current_slide = 1;
                self.current_step = 1;
            }
            Message::PreviousSlide => {
                if self.current_slide > 1 {
                    self.current_slide -= 1;
                }
                self.current_step = 1;
            }
            Message::PreviousStep => if self.current_step > 1 {
                self.current_step -= 1;
            },
            Message::NextStep => self.current_step += 1,
            Message::NextSlide => {
                self.current_slide += 1;
                self.current_step = 1;
            }
            Message::Resize => self.resize(),
            Message::Ignore => return false,
        }
        true
    }
}

impl Renderable<Presentrs> for Presentrs {
    fn view(&self) -> Html<Self> {
        html! {
            <div
                tabindex = 0,
                onkeydown = |event| Self::on_key_down(event),
                style = {"
                    position: absolute;
                    left: 0;
                    right: 0;
                    top: 0;
                    bottom: 0;
                "},
                >
                <Slides:
                    current_slide = self.current_slide,
                    current_step = self.current_step,
                    size = self.slide_size,
                    />
                <Notes:
                    current_slide = self.current_slide,
                    current_step = self.current_step,
                    />
                <Navigation:
                    on_previous_slide = |_| Message::PreviousSlide,
                    on_previous_step = |_| Message::PreviousStep,
                    on_next_step = |_| Message::NextStep,
                    on_next_slide = |_| Message::NextSlide,
                    />
            </div>
        }
    }
}
