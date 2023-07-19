use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::{
    models::{TagCreate, TagUpdate},
    services::{create_tag, delet_tag, get_tag, update_tag},
};

pub async fn tag_create(
    State(state): State<AppState>,
    tag: Json<TagCreate>,
) -> impl IntoResponse {
    let result = create_tag(state.pool, &tag).await;

    match result {
        Ok(result) => {
            (StatusCode::OK, format!("{}", result.id)).into_response()
        }
        Err(error) => {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn tag_get(
    State(state): State<AppState>,
    Path(tag_id): Path<i32>,
) -> impl IntoResponse {
    let result = get_tag(state.pool, tag_id).await;

    match result {
        Ok(result) => Json(result).into_response(),
        Err(error) => {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn tag_del(
    State(state): State<AppState>,
    Path(tag_id): Path<i32>,
) -> impl IntoResponse {
    let result = delet_tag(state.pool, tag_id).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(error) => {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn tag_update(
    State(state): State<AppState>,
    Path(tag_id): Path<i32>,
    tag: Json<TagUpdate>,
) -> impl IntoResponse {
    let result = update_tag(state.pool, tag_id, &tag).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(error) => {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
