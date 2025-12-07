use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub build_address: String,
    pub api_base_path: String,
}

pub fn config() -> Config {
    dotenv().ok();
    Config {
        build_address: env::var("BUILD_ADDRESS").expect("DATABASE_URL not set!"),
        api_base_path: env::var("API_BASE_PATH").expect("API_BASE_PATH not set!"),
    }
}