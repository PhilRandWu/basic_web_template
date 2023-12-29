use actix_web::{web, App, HttpServer};
use basic_web_template::config::Config;

#[actix_web::main]
async fn main() {
    let config_path: &'static str = "config.json";
    let config = Config::from_file(config_path);
    println!("Using configuration file from {0}", config_path);
}
