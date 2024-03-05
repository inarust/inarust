use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
mod routes;
mod controllers;
mod models;
mod configs;


#[tokio::main]
async fn main() {
    //Load config
    let config = configs::load_config();
    // Set up the MongoDB connection options
    let client_options = ClientOptions::parse(&config.mongo_string).await.unwrap();
    // Connect to the MongoDB server
    let client = Client::with_options(client_options).unwrap();
    let arc_client = Arc::new(client);
    
  
    // Start Server
    println!("Running on {}",&config.server_address);
    let listener = tokio::net::TcpListener::bind(&config.server_address).await.unwrap();
    let app = routes::route(arc_client);
    axum::serve(listener, app).await.unwrap();
}
