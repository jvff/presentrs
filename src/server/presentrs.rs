use {
    super::slide_presenter::SlidePresenter,
    axum::{
        handler, http::StatusCode, routing::BoxRoute, service,
        AddExtensionLayer, Router,
    },
    std::{convert::Infallible, io, path::Path},
    tower_http::services::ServeDir,
};

pub struct Presentrs;

impl Presentrs {
    pub fn new(path: impl AsRef<Path>) -> Router<BoxRoute> {
        Router::new()
            .nest(
                "/",
                service::get(ServeDir::new(path)).handle_error(
                    |_error: io::Error| -> Result<_, Infallible> {
                        Ok((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to serve file"),
                        ))
                    },
                ),
            )
            .route("/sync", handler::get(SlidePresenter::handler))
            .layer(AddExtensionLayer::new(SlidePresenter::new()))
            .boxed()
    }
}
