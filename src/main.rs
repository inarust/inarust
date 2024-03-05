// main.rs

mod routes;
mod controllers;
mod models;
mod configs;

use axum::Server;


#[tokio::main]
async fn main() {
    // Define Routes
    let app = routes::route();

    println!("Running on http://localhost:3000");
    
    // Start Server
    let config = configs::load_config();
    Server::bind(&config.server_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
