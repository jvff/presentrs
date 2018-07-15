use std::io;
use std::net::ToSocketAddrs;

use actix_web::{self, App};

pub struct Server;

impl Server {
    pub fn run<T>(&mut self, address: T) -> Result<(), ServerStartError>
    where
        T: ToSocketAddrs,
    {
        let server = actix_web::server::new(Self::create_app);

        server
            .bind(address)
            .map_err(ServerStartError::InvalidAddress)?
            .run();

        Ok(())
    }

    fn create_app() -> App<()> {
        let static_handler = actix_web::fs::StaticFiles::new("static");

        App::new().handler("/", static_handler)
    }
}

#[derive(Debug, Fail)]
pub enum ServerStartError {
    #[fail(display = "Invalid address to bind to")]
    InvalidAddress(#[cause] io::Error),
}
