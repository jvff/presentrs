use {
    std::convert::TryInto,
    yew::{
        format::Binary,
        prelude::*,
        services::websocket::{
            WebSocketService, WebSocketStatus, WebSocketTask,
        },
    },
};

pub struct SlideSync {
    component_link: ComponentLink<Self>,
    url: String,
    connection: Option<WebSocketTask>,
    presenting: bool,
    current_slide: usize,
    current_step: usize,
    on_update_position: Callback<(u16, u16)>,
}

#[derive(Clone, Debug, Properties)]
pub struct Properties {
    pub url: String,
    pub presenting: bool,
    pub current_slide: usize,
    pub current_step: usize,
    pub on_update_position: Callback<(u16, u16)>,
}

pub enum Message {
    Connected,
    Disconnected,
    Update { slide: u16, step: u16 },
    Ignore,
}

impl SlideSync {
    fn present(&mut self) -> ShouldRender {
        if self.presenting && self.connection.is_none() {
            self.connect();
        }

        true
    }

    fn connect(&mut self) {
        self.connection = WebSocketService::connect_binary(
            &self.url,
            self.component_link.callback(Self::ws_message_handler),
            self.component_link.callback(Self::ws_event_handler),
        )
        .ok();
    }

    fn reconnect(&mut self) -> ShouldRender {
        if self.presenting {
            self.connect();
            true
        } else {
            false
        }
    }

    fn ws_message_handler(message_bytes: Binary) -> Message {
        let message_bytes = match message_bytes {
            Ok(bytes) => bytes,
            Err(_) => return Message::Ignore,
        };

        if message_bytes.len() == 4 {
            let slide =
                u16::from_be_bytes(message_bytes[0..2].try_into().unwrap());
            let step =
                u16::from_be_bytes(message_bytes[2..4].try_into().unwrap());

            Message::Update { slide, step }
        } else {
            Message::Ignore
        }
    }

    fn ws_event_handler(event: WebSocketStatus) -> Message {
        match event {
            WebSocketStatus::Opened => Message::Connected,
            WebSocketStatus::Closed | WebSocketStatus::Error => {
                Message::Disconnected
            }
        }
    }

    fn send_position(&mut self) -> ShouldRender {
        if self.presenting {
            if let Some(connection) = self.connection.as_mut() {
                let mut message = Vec::with_capacity(4);
                let slide = self.current_slide.try_into().unwrap_or(u16::MAX);
                let step = self.current_step.try_into().unwrap_or(u16::MAX);

                message.extend(slide.to_be_bytes());
                message.extend(step.to_be_bytes());

                connection.send_binary(Ok(message));
            }
        }

        false
    }

    fn update(&mut self, slide: u16, step: u16) -> ShouldRender {
        self.on_update_position.emit((slide, step));
        false
    }

    fn apply_change<T: PartialEq>(target: &mut T, source: T) -> bool {
        if *target != source {
            *target = source;
            true
        } else {
            false
        }
    }
}

impl Component for SlideSync {
    type Message = Message;
    type Properties = Properties;

    fn create(
        properties: Self::Properties,
        component_link: ComponentLink<Self>,
    ) -> Self {
        SlideSync {
            component_link,
            url: properties.url,
            connection: None,
            presenting: properties.presenting,
            current_slide: properties.current_slide,
            current_step: properties.current_step,
            on_update_position: properties.on_update_position,
        }
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        let presenting_changed =
            Self::apply_change(&mut self.presenting, properties.presenting);

        let position_changed = Self::apply_change(
            &mut self.current_slide,
            properties.current_slide,
        ) | Self::apply_change(
            &mut self.current_step,
            properties.current_step,
        );

        if presenting_changed {
            self.present();
        }

        if position_changed {
            self.send_position();
        }

        Self::apply_change(&mut self.url, properties.url)
            | presenting_changed
            | position_changed
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::Connected => self.send_position(),
            Message::Disconnected => self.reconnect(),
            Message::Update { slide, step } => self.update(slide, step),
            Message::Ignore => false,
        }
    }

    fn view(&self) -> Html {
        if self.presenting {
            html! {
                <div style="float: left; margin: 10px">
                    <svg
                        viewBox="-70 -70 140 140"
                        style="height: 20px"
                        >
                        <circle
                            cx = 0
                            cy = 0
                            r = 35
                            style="fill: black"
                            />
                    </svg>
                </div>
            }
        } else {
            html! {
                <div style="float: left; margin: 10px">
                    <svg
                        viewBox="-70 -70 140 140"
                        style="height: 20px"
                        >
                        <path
                            d="M50,-50 L-36,0 L50,50"
                            style="fill: black"
                            />
                    </svg>
                </div>
            }
        }
    }
}
