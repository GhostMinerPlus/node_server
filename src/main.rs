use std::io;

use earth::AsConfig;
use serde::{Deserialize, Serialize};

mod state;
mod server;

#[derive(Debug, Deserialize, Serialize, Clone, AsConfig)]
struct Config {
    ip: String,
    name: String,
    port: u16,
    thread_num: u8,
    log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            name: "node".to_string(),
            port: 80,
            thread_num: 8,
            log_level: "INFO".to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    let mut arg_v: Vec<String> = std::env::args().collect();
    arg_v.remove(0);
    let file_name = if !arg_v.is_empty() && !arg_v[0].starts_with("--") {
        arg_v.remove(0)
    } else {
        "config.toml".to_string()
    };

    let mut config = Config::default();
    config.merge_by_file(&file_name);
    if !arg_v.is_empty() {
        config.merge_by_arg_v(&arg_v);
    }

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&config.log_level))
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(config.thread_num as usize)
        .build()?
        .block_on(server::Server::new(config.ip, config.port, config.name).run())
}
