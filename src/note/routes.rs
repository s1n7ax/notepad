use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::state::AppState;

use super::handlers::{note_create, note_delete, note_get, note_update};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/note",
        Router::new()
            .route("/", post(note_create))
            .route("/:note_id", get(note_get))
            .route("/:note_id", delete(note_delete))
            .route("/:note_id", put(note_update)),
    )
}
