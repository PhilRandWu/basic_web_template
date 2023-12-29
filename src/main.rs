use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer};
use basic_web_template::AppState;
use basic_web_template::config::Config;
use basic_web_template::dao::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Start Server");
    // Config
    let config_path: &'static str = "config.json";
    let config = Config::from_file(config_path);
    println!("Using configuration file from {0}", config_path);


    // Connect to the database
    let db_context = Database::new(&config.get_database_url()).await;
    println!("Connected to database: {0}", config.get_database_url());

    // Instantiate the app_state. This application state will be cloned for each Actix thread but
    // the Arc of the DbContext will be reused in each Actix thread.
    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    // Start the web application
    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
    })
        .bind(config.get_app_url())?;
    println!("listening on: {0}", config.get_app_url());

    app.run().await
}
