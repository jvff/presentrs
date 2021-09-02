use {
    std::{convert::TryInto, mem},
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
    state: State,
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
    ToggleSync,
    Update { slide: u16, step: u16 },
    Ignore,
}

enum State {
    Offline,
    Syncing(WebSocketTask),
    Presenting(WebSocketTask),
}

impl SlideSync {
    fn present(&mut self) -> ShouldRender {
        let previous_state = mem::replace(&mut self.state, State::Offline);

        if self.presenting {
            self.state = match previous_state {
                State::Offline => State::Presenting(self.connect()),
                State::Syncing(connection) | State::Presenting(connection) => {
                    State::Presenting(connection)
                }
            };
        } else {
            self.state = match previous_state {
                State::Offline => State::Offline,
                State::Syncing(connection) | State::Presenting(connection) => {
                    State::Syncing(connection)
                }
            };
        }

        true
    }

    fn toggle_sync(&mut self) -> ShouldRender {
        let previous_state = mem::replace(&mut self.state, State::Offline);

        self.state = match previous_state {
            State::Offline => State::Syncing(self.connect()),
            State::Syncing(_) => State::Offline,
            State::Presenting(connection) => State::Syncing(connection),
        };

        self.presenting = false;

        true
    }

    fn connect(&mut self) -> WebSocketTask {
        for _attempt in 1..10 {
            let connection = WebSocketService::connect_binary(
                &self.url,
                self.component_link.callback(Self::ws_message_handler),
                self.component_link.callback(Self::ws_event_handler),
            );

            if let Ok(connection) = connection {
                return connection;
            }
        }

        panic!("Failed to connect for synchronization");
    }

    fn reconnect(&mut self) -> ShouldRender {
        match mem::replace(&mut self.state, State::Offline) {
            State::Offline => {}
            State::Syncing(_) => self.state = State::Syncing(self.connect()),
            State::Presenting(_) => {
                self.state = State::Presenting(self.connect())
            }
        }

        false
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
        let slide = self.current_slide.try_into().unwrap_or(u16::MAX);
        let step = self.current_step.try_into().unwrap_or(u16::MAX);

        self.state.send_position(slide, step);

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

impl State {
    pub fn send_position(&mut self, slide: u16, step: u16) {
        match self {
            State::Offline | State::Syncing(_) => {}
            State::Presenting(connection) => {
                let mut message = Vec::with_capacity(4);

                message.extend(slide.to_be_bytes());
                message.extend(step.to_be_bytes());

                connection.send_binary(Ok(message));
            }
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
            state: State::Offline,
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
            Message::ToggleSync => self.toggle_sync(),
            Message::Update { slide, step } => self.update(slide, step),
            Message::Ignore => false,
        }
    }

    fn view(&self) -> Html {
        let toggle_sync = self.component_link.callback(|_| Message::ToggleSync);

        match self.state {
            State::Offline => {
                html! {
                    <div style="float: left; margin: 10px">
                        <svg
                            viewBox="-70 -70 140 140"
                            style="height: 20px"
                            onclick = toggle_sync
                            >
                            <path
                                d="M-60,-60 L46,0 L-60,60"
                                style="fill: black"
                                />
                        </svg>
                    </div>
                }
            }
            State::Syncing(_) => {
                html! {
                    <div style="float: left; margin: 10px">
                        <svg
                            viewBox="-70 -70 140 140"
                            style="height: 20px"
                            onclick = toggle_sync
                            >
                            <rect
                                x = "-70"
                                y = "-70"
                                width = 40
                                height = 140
                                style="fill: black"
                                />
                            <rect
                                x = 30
                                y = "-70"
                                width = 40
                                height = 140
                                style="fill: black"
                                />
                        </svg>
                    </div>
                }
            }
            State::Presenting(_) => {
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
            }
        }
    }
}
