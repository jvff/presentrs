mod navigation;
mod notes;
mod slide_size;
mod slides;

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
    PreviousSlide,
    PreviousStep,
    NextSlide,
    NextStep,
    Resize,
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
        }
        true
    }
}

impl Renderable<Presentrs> for Presentrs {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
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
