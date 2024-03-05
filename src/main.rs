mod routes;
mod controllers;
mod models;
mod configs;


#[tokio::main]
async fn main() {
    // Define Routes
    let app = routes::route();
  
    // Start Server
    let config = configs::load_config();
    println!("Running on {}",&config.server_address);
    let listener = tokio::net::TcpListener::bind(&config.server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
