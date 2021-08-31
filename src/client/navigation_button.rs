use {once_cell::sync::Lazy, yew::prelude::*};

pub struct NavigationButton {
    direction: Direction,
    target: Target,
    on_click: Option<Callback<MouseEvent>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Properties {
    pub direction: Direction,
    pub target: Target,
    pub on_click: Option<Callback<()>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    Step,
    Slide,
}

impl NavigationButton {
    fn prepare_callback(
        callback: Option<Callback<()>>,
    ) -> Option<Callback<MouseEvent>> {
        callback.map(|callback| callback.reform(|_| ()))
    }
}

impl Component for NavigationButton {
    type Message = ();
    type Properties = Properties;

    fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavigationButton {
            direction: properties.direction,
            target: properties.target,
            on_click: Self::prepare_callback(properties.on_click),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.direction = properties.direction;
        self.target = properties.target;
        self.on_click = Self::prepare_callback(properties.on_click);

        true
    }

    fn view(&self) -> Html {
        html! {
            <div style="float: left; margin: 10px">
                <svg
                    viewBox="-70 -70 140 140"
                    style="height: 20px"
                    onclick=&self.on_click
                    >
                    <path
                        d="M50,-50 L-36,0 L50,50"
                        style=self.target.style()
                        transform=self.direction.transform()
                        />
                </svg>
            </div>
        }
    }
}

impl Direction {
    fn transform(&self) -> Option<&'static str> {
        match self {
            Direction::Forward => Some("scale(-1,1)"),
            Direction::Backward => None,
        }
    }
}

impl Target {
    const STEP_STYLE: &'static str = "\
        fill: none;\
        stroke: black;\
        stroke-width: 20;\
        stroke-dasharray: 2,31;\
        stroke-linecap: round;\
        stroke-linejoin: round;\
    ";

    const SLIDE_STYLE: Lazy<&'static str> = Lazy::new(|| {
        let (slide_style_end, _) = Target::STEP_STYLE
            .chars()
            .enumerate()
            .filter(|(_, character)| *character == ';')
            .nth(2)
            .expect("STEP_STYLE couldn't be mapped into SLIDE_STYLE");

        &Target::STEP_STYLE[..slide_style_end]
    });

    fn style(&self) -> &'static str {
        match self {
            Target::Step => Self::STEP_STYLE,
            Target::Slide => *Self::SLIDE_STYLE,
        }
    }
}
