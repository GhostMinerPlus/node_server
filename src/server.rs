//! Server that provides services.

use std::{io, sync::Arc, time::Duration};

use axum::{routing, Router};
use tokio::time;

use crate::{star, state};

mod service;

async fn start_task_v(
    moon_server_v: Vec<String>,
    name: String,
    path: String,
    port: u16,
) -> io::Result<()> {
    if moon_server_v.is_empty() {
        return Ok(());
    }
    log::debug!("moon_server_v is not empty");
    tokio::spawn(async move {
        log::debug!("starting moon_server_v loop");
        loop {
            time::sleep(Duration::from_secs(10)).await;
            if let Err(e) = star::report_uri(&name, port, &path, &moon_server_v).await {
                log::error!("{e}");
            }
        }
    });
    Ok(())
}

async fn serve(ip: &str, port: u16, name: &str) -> io::Result<()> {
    // build our application with a route
    let app = Router::new()
        .route(&format!("/{}/", name), routing::get(service::http_index))
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
    moon_server_v: Vec<String>,
}

impl Server {
    pub fn new(ip: String, port: u16, name: String, moon_server_v: Vec<String>) -> Self {
        Self {
            ip,
            port,
            name,
            moon_server_v,
        }
    }

    pub async fn run(self) -> io::Result<()> {
        start_task_v(
            self.moon_server_v.clone(),
            self.name.clone(),
            format!("/{}/", self.name),
            self.port,
        )
        .await?;
        serve(&self.ip, self.port, &self.name).await
    }
}
