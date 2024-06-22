use std::env;

use route::AppState;
use utils::config::read_config_file;

mod route;
mod utils;

#[tokio::main]
async fn main() {
    println!("hello world!");

    let config_path = format!("{}/config.toml", env::current_dir().unwrap().display());
    let config = read_config_file(config_path.as_str());
    // config.logger_init();
    // info!("Logger started with level: {}", &config.log.level);

    let http_state = AppState {};

    let http_server = config.http_init(http_state).await;
    // info!(
    //     "HTTP server started at {}:{}",
    //     &config.http.host, &config.http.port
    // );
    http_server.await.unwrap();
}
