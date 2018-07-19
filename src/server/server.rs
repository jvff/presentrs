use std::io;
use std::net::ToSocketAddrs;
use std::path::PathBuf;

use actix_web::{self, App};

pub struct Server {
    static_dir: PathBuf,
}

impl Server {
    pub fn new<P>(static_dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Server {
            static_dir: static_dir.into(),
        }
    }

    pub fn run<T>(&mut self, address: T) -> Result<(), ServerStartError>
    where
        T: ToSocketAddrs,
    {
        let static_dir = self.static_dir.clone();
        let server = actix_web::server::new(move || {
            Self::create_app(static_dir.clone())
        });

        server
            .bind(address)
            .map_err(ServerStartError::InvalidAddress)?
            .run();

        Ok(())
    }

    fn create_app(static_dir: PathBuf) -> App<()> {
        let static_handler = actix_web::fs::StaticFiles::new(static_dir);

        App::new().handler("/", static_handler)
    }
}

#[derive(Debug, Fail)]
pub enum ServerStartError {
    #[fail(display = "Invalid address to bind to")]
    InvalidAddress(#[cause] io::Error),
}
