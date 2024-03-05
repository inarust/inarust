mod routes;
mod controllers;
mod models;
mod configs;

use axum::Server;


#[tokio::main]
async fn main() {
    // Define Routes
    let app = routes::route();
  
    // Start Server
    let config = configs::load_config();
    println!("Running on {}",&config.server_address);
    Server::bind(&config.server_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
