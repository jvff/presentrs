use yew::prelude::*;

pub struct Presentrs;

impl Component for Presentrs {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Presentrs
    }

    fn update(&mut self, _message: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Presentrs> for Presentrs {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <iframe src="/notes.html",/>
            </div>
        }
    }
}
