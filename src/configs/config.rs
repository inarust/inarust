pub struct AppConfig {
    pub server_address: String,
    pub mongo_string:String,
    // ... other configuration parameters ...
}

pub fn load_config() -> AppConfig {
    // Logic to load configuration from a file, environment variables, etc.
    AppConfig {
        server_address: "0.0.0.0:3000".to_string(),
        mongo_string:"mongodb://localhost:27017".to_string(),
        // ... initialize other configuration parameters ...
    }
}
