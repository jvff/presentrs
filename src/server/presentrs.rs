use {
    axum::{http::StatusCode, routing::BoxRoute, service::get, Router},
    std::{convert::Infallible, io, path::Path},
    tower_http::services::ServeDir,
};

pub struct Presentrs;

impl Presentrs {
    pub fn new(path: impl AsRef<Path>) -> Router<BoxRoute> {
        Router::new()
            .nest(
                "/",
                get(ServeDir::new(path)).handle_error(
                    |_error: io::Error| -> Result<_, Infallible> {
                        Ok((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to serve file"),
                        ))
                    },
                ),
            )
            .boxed()
    }
}
