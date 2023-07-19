use super::models::{Tag, TagCreate, TagInserted, TagUpdate};
use sqlx::{postgres::PgQueryResult, PgPool};
use std::error::Error;

type Result<T> = core::result::Result<T, Box<dyn Error>>;

pub async fn create_tag(pool: PgPool, tag: &TagCreate) -> Result<TagInserted> {
    sqlx::query_as!(
        TagInserted,
        "INSERT INTO tags (name) VALUES ($1) RETURNING id",
        tag.name
    )
    .fetch_one(&pool)
    .await
    .map_err(Into::into)
}

pub async fn delet_tag(pool: PgPool, tag_id: i32) -> Result<PgQueryResult> {
    sqlx::query!("DELETE FROM tags WHERE id = $1", tag_id)
        .execute(&pool)
        .await
        .map_err(Into::into)
}

pub async fn update_tag(
    pool: PgPool,
    tag_id: i32,
    tag: &TagUpdate,
) -> Result<PgQueryResult> {
    sqlx::query!("UPDATE tags SET name = $1 where id = $2", tag.name, tag_id)
        .execute(&pool)
        .await
        .map_err(Into::into)
}
pub async fn get_tag(pool: PgPool, tag_id: i32) -> Result<Tag> {
    sqlx::query_as!(Tag, "SELECT * FROM tags WHERE id = $1", tag_id,)
        .fetch_one(&pool)
        .await
        .map_err(Into::into)
}
