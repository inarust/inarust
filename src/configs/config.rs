use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
pub struct AppConfig {
    pub server_address: String,
    pub arc_client:Arc<Client>,
    // ... other configuration parameters ...
}

pub async fn load_config() -> AppConfig {
    // Set up the MongoDB connection options
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    // Connect to the MongoDB server
    let client = Client::with_options(client_options).unwrap();
    let arc_client = Arc::new(client);
    // Logic to load configuration from a file, environment variables, etc.
    AppConfig {
        server_address: "0.0.0.0:3000".to_string(),
        arc_client:arc_client,
        // ... initialize other configuration parameters ...
    }
}
