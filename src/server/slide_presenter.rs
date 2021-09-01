use {
    axum::{
        extract::{
            ws::{Message, WebSocket, WebSocketUpgrade},
            Extension,
        },
        response::IntoResponse,
    },
    futures_util::{select, FutureExt, StreamExt},
    std::{convert::TryInto, sync::Arc},
    tokio::sync::broadcast,
};

pub struct SlidePresenter {
    position: broadcast::Sender<(u16, u16)>,
}

impl SlidePresenter {
    pub fn new() -> Arc<Self> {
        let (position, _) = broadcast::channel(1);

        Arc::new(SlidePresenter { position })
    }

    pub async fn handler(
        web_socket_upgrade: WebSocketUpgrade,
        Extension(presenter): Extension<Arc<SlidePresenter>>,
    ) -> impl IntoResponse {
        let position_receiver = presenter.position.subscribe();

        web_socket_upgrade.on_upgrade(|web_socket| {
            presenter.handle(web_socket, position_receiver)
        })
    }

    async fn handle(
        self: Arc<Self>,
        mut web_socket: WebSocket,
        mut position_receiver: broadcast::Receiver<(u16, u16)>,
    ) {
        loop {
            let result = select! {
                message = web_socket.next().fuse() => {
                    self.handle_message(message)
                }
                position = position_receiver.recv().fuse() => {
                    Self::handle_position(position, &mut web_socket).await
                }
            };

            if result.is_err() {
                break;
            }
        }
    }

    fn handle_message(
        &self,
        maybe_message: Option<Result<Message, axum::Error>>,
    ) -> Result<(), ()> {
        let message = maybe_message.ok_or(())?.map_err(|_| ())?;

        if let Message::Binary(message_bytes) = message {
            if message_bytes.len() == 4 {
                let slide_index_bytes = message_bytes[0..2].try_into().unwrap();
                let step_index_bytes = message_bytes[2..4].try_into().unwrap();

                let slide_index = u16::from_be_bytes(slide_index_bytes);
                let step_index = u16::from_be_bytes(step_index_bytes);

                self.position
                    .send((slide_index, step_index))
                    .map_err(|_| ())?;
            }
        }

        Ok(())
    }

    async fn handle_position(
        position: Result<(u16, u16), broadcast::error::RecvError>,
        web_socket: &mut WebSocket,
    ) -> Result<(), ()> {
        if let Ok((slide, step)) = position {
            let mut message_bytes = Vec::with_capacity(4);

            message_bytes.extend(slide.to_be_bytes());
            message_bytes.extend(step.to_be_bytes());

            web_socket
                .send(Message::Binary(message_bytes))
                .await
                .map_err(|_| ())
        } else {
            Ok(())
        }
    }
}
