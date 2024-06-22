use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub develop: InnerConfig,
    pub production: InnerConfig,
}

#[derive(Deserialize, Debug)]
pub struct InnerConfig {
    // pub log: LogConfig,
    pub http: HttpConfig,
    // pub database: DatabaseConfig,
}

// #[derive(Deserialize, Debug)]
// pub struct LogConfig {
//     pub level: String,
// }

#[derive(Deserialize, Debug)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}
// #[derive(Deserialize, Debug)]
// pub struct DatabaseConfig {
//     pub host: String,
//     pub port: u16,
//     pub dbnamespace: String,
//     pub dbname: String,
//     pub user: String,
//     pub password: String,
// }

pub fn read_config_file(config_file: &str) -> InnerConfig {
    let string_toml_file = std::fs::read_to_string(config_file).unwrap();
    let read_all_config: Config = toml::from_str(&string_toml_file).unwrap();
    let mode = env::var("MODE").unwrap_or("develop".to_string());
    let config = match mode.as_str() {
        "develop" => read_all_config.develop,
        "production" => read_all_config.production,
        _ => unreachable!("MODE must be develop or production; And default is develop."),
    };

    config
}
