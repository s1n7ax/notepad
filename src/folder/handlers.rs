use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use axum::response::Json;

use super::{
    models::{FolderCreate, FolderUpdate},
    services::{create_folder, delete_folder, get_folder, update_folder},
};

pub async fn folder_create(
    State(state): State<AppState>,
    folder: Json<FolderCreate>,
) -> impl IntoResponse {
    let result = create_folder(state.pool, &folder.0).await;

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

pub async fn folder_get(
    State(state): State<AppState>,
    Path(folder_id): Path<i32>,
) -> impl IntoResponse {
    let result = get_folder(state.pool, folder_id).await;

    match result {
        Ok(result) => Json(result).into_response(),
        Err(error) => {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn folder_del(
    State(state): State<AppState>,
    Path(folder_id): Path<i32>,
) -> impl IntoResponse {
    let result = delete_folder(state.pool, folder_id).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(error) => {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn folder_update(
    State(state): State<AppState>,
    Path(folder_id): Path<i32>,
    folder: Json<FolderUpdate>,
) -> impl IntoResponse {
    let result = update_folder(state.pool, folder_id, &folder.0).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(error) => {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
