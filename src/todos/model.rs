use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodo {
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodo {
    pub description: Option<String>,
    pub done: Option<bool>,
}
