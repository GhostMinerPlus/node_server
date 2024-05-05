//! Start server

use std::{io, time::Duration};

use earth::AsConfig;
use edge_lib::{data::AsDataManager, AsEdgeEngine, EdgeEngine};
use node_server::{data::DataManager, server};
use tokio::time;

// Public
#[derive(serde::Deserialize, serde::Serialize, AsConfig, Clone, Debug)]
/// Config
struct Config {
    /// Default: node_server
    name: String,
    /// Default: 0.0.0.0
    ip: String,
    /// Default: 80
    port: u16,
    /// Default: info
    log_level: String,
    /// Default: 8
    thread_num: u8,
    moon_servers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "node_server".to_string(),
            ip: "0.0.0.0".to_string(),
            port: 80,
            log_level: "info".to_string(),
            thread_num: 8,
            moon_servers: Vec::new(),
        }
    }
}

fn main() -> io::Result<()> {
    // Parse config
    let mut config = Config::default();
    let mut arg_v: Vec<String> = std::env::args().collect();
    arg_v.remove(0);
    let file_name = if !arg_v.is_empty() && !arg_v[0].starts_with("--") {
        arg_v.remove(0)
    } else {
        "config.toml".to_string()
    };
    config.merge_by_file(&file_name);
    if !arg_v.is_empty() {
        config.merge_by_arg_v(&arg_v);
    }
    config.merge_by_env(&format!("{}", config.name));
    // Config log
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&config.log_level))
        .init();
    log::debug!("{:?}", config);
    // Run server
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.thread_num as usize)
        .enable_all()
        .build()?
        .block_on(async {
            let dm = DataManager::new();
            let mut edge_engine = EdgeEngine::new(dm.divide());
            // config.ip, config.port, config.name
            let base_script = [
                format!("root->name = = {} _", config.name),
                format!("root->ip = = {} _", config.ip),
                format!("root->port = = {} _", config.port),
            ]
            .join("\\n");
            let option_script = config
                .moon_servers
                .into_iter()
                .map(|moon_server| format!("root->moon_server += = {moon_server} _\\n"))
                .reduce(|acc, line| format!("{acc}{line}"))
                .unwrap_or(String::new());
            edge_engine
                .execute(
                    &json::parse(&format!("{{\"{base_script}\\n{option_script}\": null}}"))
                        .unwrap(),
                )
                .await?;
            edge_engine.commit().await?;

            tokio::spawn(server::HttpConnector::new(dm.divide()).run());
            tokio::spawn(server::HttpServer::new(dm.divide()).run());
            loop {
                time::sleep(Duration::from_secs(10)).await;
                log::info!("alive");
            }
        })
}
