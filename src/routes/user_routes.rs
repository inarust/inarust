use axum::{
    routing::{get, post,delete},
    Router,
};

use crate::controllers;

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(controllers::create_user))
        .route("/users", get(controllers::list_users))
        .route("/item/:id", get(controllers::show_item))
        .route("/add-item", post(controllers::add_item))
        .route("/delete-user/:user_id", delete(controllers::delete_user))
}
