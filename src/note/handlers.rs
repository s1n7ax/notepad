use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::state::AppState;

use super::{
    models::{NoteCreate, NoteUpdate},
    services::{create_note, delete_note, get_note, update_note},
};

pub async fn note_create(
    State(state): State<AppState>,
    note: Json<NoteCreate>,
) -> impl IntoResponse {
    let result = create_note(state.pool, note.0).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn note_delete(
    State(state): State<AppState>,
    Path(note_id): Path<i32>,
) -> impl IntoResponse {
    let result = delete_note(state.pool, note_id).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn note_get(
    State(state): State<AppState>,
    Path(note_id): Path<i32>,
) -> impl IntoResponse {
    let result = get_note(state.pool, note_id).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn note_update(
    State(state): State<AppState>,
    Path(note_id): Path<i32>,
    note: Json<NoteUpdate>,
) -> impl IntoResponse {
    let result = update_note(state.pool, note_id, note.0).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
