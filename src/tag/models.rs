use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TagCreate {
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TagUpdate {
    pub name: String,
}

pub struct TagInserted {
    pub id: i32,
}
