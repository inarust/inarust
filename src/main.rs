// main.rs

mod routes;
mod controllers;
mod models;

use axum::
    Server
;

#[tokio::main]
async fn main() {
    // Define Routes
    let app = routes::user_routes();

    println!("Running on http://localhost:3000");
    
    // Start Server
    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
