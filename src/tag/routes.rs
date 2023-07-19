use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    state::AppState,
    tag::handlers::{tag_create, tag_del, tag_get, tag_update},
};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/tag",
        Router::new()
            .route("/", post(tag_create))
            .route("/:tag_id", get(tag_get))
            .route("/:tag_id", put(tag_update))
            .route("/:tag_id", delete(tag_del)),
    )
}
