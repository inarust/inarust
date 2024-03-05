use serde::{Serialize,Deserialize};

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

// A struct for query parameters
#[derive(Deserialize)]
pub struct Page {
    pub number: u32,
}


// A struct for the JSON body
#[derive(Deserialize)]
pub struct Item {
    pub title: String,
}

#[derive(Serialize)]
pub struct Userx {
    pub id: u64,
    pub name: String,
}