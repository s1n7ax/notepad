use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: Value,
    pub parent_id: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NoteCreate {
    pub title: String,
    pub content: Value,
    pub parent_id: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NoteUpdate {
    pub title: String,
    pub content: Value,
    pub parent_id: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NoteInserted {
    pub id: i32,
}
