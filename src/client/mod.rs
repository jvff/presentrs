mod notes;

use yew::prelude::*;

use self::notes::Notes;

pub enum Message {
    PreviousSlide,
    PreviousStep,
    NextSlide,
    NextStep,
}

pub struct Presentrs {
    current_slide: usize,
    current_step: usize,
}

impl Component for Presentrs {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Presentrs {
            current_slide: 1,
            current_step: 1,
        }
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
            Message::NextSlide => {
                self.current_slide += 1;
                self.current_step = 1;
            }
            Message::NextStep => self.current_step += 1,
        }
        true
    }
}

impl Renderable<Presentrs> for Presentrs {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <Notes:
                    current_slide = self.current_slide,
                    current_step = self.current_step,
                    />
                <form onsubmit="return false;",>
                    <button type="submit", onclick=|_| Message::PreviousSlide,>
                        {"Previous slide"}
                    </button>
                    <button type="submit", onclick=|_| Message::PreviousStep,>
                        {"Previous step"}
                    </button>
                    <button type="submit", onclick=|_| Message::NextStep,>
                        {"Next step"}
                    </button>
                    <button type="submit", onclick=|_| Message::NextSlide,>
                        {"Next slide"}
                    </button>
                </form>
            </div>
        }
    }
}
