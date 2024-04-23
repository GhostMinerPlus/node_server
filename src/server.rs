//! Server that provides services.

use std::{io, sync::Arc};

use axum::{routing, Router};

use crate::state;

mod service;

async fn serve(ip: &str, port: u16, name: &str) -> io::Result<()> {
    // build our application with a route
    let app = Router::new()
        .route(
            &format!("/{}/execute", name),
            routing::post(service::http_index),
        )
        .with_state(Arc::new(state::ServerState {}));

    // run our app with hyper, listening globally on port 3000
    let address = format!("{}:{}", ip, port);
    log::info!("serving at {address}/{}", name);
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await
}

// Public
pub struct Server {
    ip: String,
    name: String,
    port: u16,
}

impl Server {
    pub fn new(ip: String, port: u16, name: String) -> Self {
        Self {
            ip,
            port,
            name
        }
    }

    pub async fn run(self) -> io::Result<()> {
        serve(&self.ip, self.port, &self.name).await
    }
}
