use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct FolderUpdate {
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct FolderCreate {
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Clone)]
pub struct FolderInserted {
    pub id: i32,
}
