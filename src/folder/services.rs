use crate::folder::models::FolderInserted;

use super::models::{Folder, FolderCreate, FolderUpdate};
use sqlx::{postgres::PgQueryResult, PgPool};
use std::error::Error;

type Result<T> = core::result::Result<T, Box<dyn Error>>;

pub async fn create_folder(
    pool: PgPool,
    folder: &FolderCreate,
) -> Result<FolderInserted> {
    sqlx::query_as!(
        FolderInserted,
        "INSERT INTO folders (name, parent_id) VALUES ($1, $2) RETURNING id",
        folder.name,
        folder.parent_id
    )
    .fetch_one(&pool)
    .await
    .map_err(Into::into)
}

pub async fn delete_folder(
    pool: PgPool,
    folder_id: i32,
) -> Result<PgQueryResult> {
    sqlx::query!("DELETE FROM folders WHERE id=$1;", folder_id)
        .execute(&pool)
        .await
        .map_err(Into::into)
}

pub async fn update_folder(
    pool: PgPool,
    folder_id: i32,
    new_folder: &FolderUpdate,
) -> Result<PgQueryResult> {
    sqlx::query!(
        "UPDATE folders SET name = $1, parent_id = $2 WHERE id = $3",
        new_folder.name,
        new_folder.parent_id,
        folder_id
    )
    .execute(&pool)
    .await
    .map_err(Into::into)
}

pub async fn get_folder(pool: PgPool, folder_id: i32) -> Result<Folder> {
    sqlx::query_as!(Folder, "SELECT * FROM folders WHERE id = $1", folder_id,)
        .fetch_one(&pool)
        .await
        .map_err(Into::into)
}
