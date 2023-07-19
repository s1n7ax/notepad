use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    folder::handlers::{folder_create, folder_del, folder_get, folder_update},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/folder",
        Router::new()
            .route("/", post(folder_create))
            .route("/:folder_id", get(folder_get))
            .route("/:folder_id", put(folder_update))
            .route("/:folder_id", delete(folder_del)),
    )
}
