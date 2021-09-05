use {
    axum::{
        extract::{
            ws::{Message, WebSocket, WebSocketUpgrade},
            Extension,
        },
        response::IntoResponse,
    },
    derive_more::{Display, Error, From},
    futures_util::{select, FutureExt, StreamExt},
    std::{
        convert::TryInto,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    },
    tokio::sync::broadcast,
    tracing::{error, info, trace, trace_span, Span},
};

pub struct SlidePresenter {
    position: broadcast::Sender<(usize, u16, u16)>,
    id_counter: AtomicUsize,
}

impl SlidePresenter {
    pub fn new() -> Arc<Self> {
        let (position, _) = broadcast::channel(1);

        Arc::new(SlidePresenter {
            position,
            id_counter: AtomicUsize::new(0),
        })
    }

    pub async fn handler(
        web_socket_upgrade: WebSocketUpgrade,
        Extension(presenter): Extension<Arc<SlidePresenter>>,
    ) -> impl IntoResponse {
        let position_receiver = presenter.position.subscribe();
        let id = presenter.id_counter.fetch_add(1, Ordering::Relaxed);

        web_socket_upgrade.on_upgrade(move |web_socket| {
            presenter.handle(id, web_socket, position_receiver)
        })
    }

    async fn handle(
        self: Arc<Self>,
        id: usize,
        mut web_socket: WebSocket,
        mut position_receiver: broadcast::Receiver<(usize, u16, u16)>,
    ) {
        let span = trace_span!("WebSocket handler #{}", id);

        loop {
            let result = select! {
                message = web_socket.next().fuse() => {
                    self.handle_message(id, message, &span)
                }
                position = position_receiver.recv().fuse() => {
                    Self::handle_position(id, position, &mut web_socket, &span).await
                }
            };

            if let Err(error) = result {
                error.report(&span);
                break;
            }
        }
    }

    fn handle_message(
        &self,
        id: usize,
        maybe_message: Option<Result<Message, axum::Error>>,
        span: &Span,
    ) -> Result<(), Error> {
        let message = maybe_message
            .ok_or(Error::Disconnected)?
            .map_err(Error::Receive)?;

        if let Message::Binary(message_bytes) = message {
            if message_bytes.len() == 4 {
                let slide_index_bytes = message_bytes[0..2].try_into().unwrap();
                let step_index_bytes = message_bytes[2..4].try_into().unwrap();

                let slide_index = u16::from_be_bytes(slide_index_bytes);
                let step_index = u16::from_be_bytes(step_index_bytes);

                span.in_scope(|| {
                    trace!("Received {}:{}", slide_index, step_index)
                });

                self.position.send((id, slide_index, step_index))?;
            }
        }

        Ok(())
    }

    async fn handle_position(
        id: usize,
        position: Result<(usize, u16, u16), broadcast::error::RecvError>,
        web_socket: &mut WebSocket,
        span: &Span,
    ) -> Result<(), Error> {
        if let Ok((sender_id, slide, step)) = position {
            if sender_id != id {
                let mut message_bytes = Vec::with_capacity(4);

                message_bytes.extend(slide.to_be_bytes());
                message_bytes.extend(step.to_be_bytes());

                span.in_scope(|| trace!("Sending {}:{}", slide, step));

                web_socket
                    .send(Message::Binary(message_bytes))
                    .await
                    .map_err(Error::Send)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display(fmt = "Client has disconnected")]
    Disconnected,

    #[display(fmt = "Failed to synchronize position internally")]
    Internal(broadcast::error::SendError<(usize, u16, u16)>),

    #[display(fmt = "Failed to receive updated position from client")]
    #[from(ignore)]
    Receive(axum::Error),

    #[display(fmt = "Failed to send synchronized position to a client")]
    #[from(ignore)]
    Send(axum::Error),
}

impl Error {
    pub fn report(&self, span: &Span) {
        span.in_scope(|| match self {
            Error::Disconnected => info!("{}", self),
            Error::Internal(_) | Error::Receive(_) | Error::Send(_) => {
                error!("{}", self)
            }
        })
    }
}
