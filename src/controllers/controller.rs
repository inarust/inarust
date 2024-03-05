use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::{Path, Query},
    Json,
};

use crate::models::{User,Page,Userx, Item};
// Handler for /create-user
pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}
// Handler for /users
pub async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        },
    ];
    Json(users)
}

// A handler to demonstrate path and query extractors
pub async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}

// A handler to demonstrate the JSON body extractor
pub async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item: {}", item.title)
}

// Define a handler that performs an operation and may return an error
pub async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<Userx>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(Userx {
            id: user_id,
            name: "Deleted User".into(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )),
    }
}

// Hypothetical async function to delete a user by ID
async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    // Simulate an error for demonstration
    if user_id == 1 {
        Err("User cannot be deleted.".to_string())
    } else {
        // Logic to delete a user...
        Ok(())
    }
}
