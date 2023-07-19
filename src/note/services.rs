use crate::note::models::NoteInserted;

use super::models::{Note, NoteCreate, NoteUpdate};
use sqlx::{postgres::PgQueryResult, query, query_as, PgPool};
use std::error::Error;

type DResult<T> = core::result::Result<T, Box<dyn Error>>;

pub async fn create_note(
    pool: PgPool,
    note: NoteCreate,
) -> DResult<NoteInserted> {
    sqlx::query_as!(
        NoteInserted,
        "INSERT INTO notes (title, content, parent_id) VALUES ($1, $2, $3) RETURNING id",
        note.title,
        note.content,
        note.parent_id
    )
    .fetch_one(&pool)
    .await
    .map_err(Into::into)
}

pub async fn delete_note(pool: PgPool, note_id: i32) -> DResult<PgQueryResult> {
    query!("DELETE FROM notes WHERE id = $1", note_id)
        .execute(&pool)
        .await
        .map_err(Into::into)
}

pub async fn update_note(
    pool: PgPool,
    note_id: i32,
    note: NoteUpdate,
) -> DResult<PgQueryResult> {
    query!(
        "UPDATE notes SET title = $1, content = $2, parent_id = $3 WHERE id = $4",
        note.title,
        note.content,
        note.parent_id,
        note_id
    )
    .execute(&pool)
    .await
    .map_err(Into::into)
}

pub async fn get_note(pool: PgPool, note_id: i32) -> DResult<Note> {
    query_as!(Note, "SELECT * FROM notes WHERE id = $1", note_id)
        .fetch_one(&pool)
        .await
        .map_err(Into::into)
}
