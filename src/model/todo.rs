use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Clone, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateTodo {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateTodo {
    pub id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_done: Option<bool>,
}
