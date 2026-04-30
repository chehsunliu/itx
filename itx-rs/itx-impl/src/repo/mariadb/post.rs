use std::collections::HashMap;

use async_trait::async_trait;
use itx_contract::repo::error::RepoError;
use itx_contract::repo::post::{
    AuthorId, ListPostsQuery, NewPost, Post, PostId, PostPatch, PostRepo,
};
use sqlx::{MySql, MySqlPool, Transaction};
use uuid::Uuid;

pub struct MariaDbPostRepo {
    pool: MySqlPool,
}

impl MariaDbPostRepo {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

fn err<E: std::fmt::Display>(e: E) -> RepoError {
    RepoError::Unknown(e.to_string())
}

fn parse_author(s: &str) -> Result<AuthorId, RepoError> {
    Uuid::parse_str(s).map_err(err)
}

async fn upsert_tags(
    tx: &mut Transaction<'_, MySql>,
    names: &[String],
) -> Result<Vec<i64>, RepoError> {
    let mut ids = Vec::with_capacity(names.len());
    for name in names {
        sqlx::query("INSERT IGNORE INTO tags (name) VALUES (?)")
            .bind(name)
            .execute(&mut **tx)
            .await
            .map_err(err)?;
        let id: i64 = sqlx::query_scalar("SELECT id FROM tags WHERE name = ?")
            .bind(name)
            .fetch_one(&mut **tx)
            .await
            .map_err(err)?;
        ids.push(id);
    }
    Ok(ids)
}

async fn link_post_tags(
    tx: &mut Transaction<'_, MySql>,
    post_id: PostId,
    tag_ids: &[i64],
) -> Result<(), RepoError> {
    for tid in tag_ids {
        sqlx::query("INSERT IGNORE INTO post_tags (post_id, tag_id) VALUES (?, ?)")
            .bind(post_id)
            .bind(tid)
            .execute(&mut **tx)
            .await
            .map_err(err)?;
    }
    Ok(())
}

async fn fetch_tags_for(
    pool: &MySqlPool,
    post_ids: &[PostId],
) -> Result<HashMap<PostId, Vec<String>>, RepoError> {
    if post_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let placeholders = vec!["?"; post_ids.len()].join(",");
    let sql = format!(
        "SELECT pt.post_id, t.name \
         FROM post_tags pt JOIN tags t ON pt.tag_id = t.id \
         WHERE pt.post_id IN ({placeholders}) \
         ORDER BY t.name"
    );
    let mut q = sqlx::query_as::<_, (PostId, String)>(&sql);
    for pid in post_ids {
        q = q.bind(pid);
    }
    let rows = q.fetch_all(pool).await.map_err(err)?;

    let mut map: HashMap<PostId, Vec<String>> = HashMap::new();
    for (pid, name) in rows {
        map.entry(pid).or_default().push(name);
    }
    Ok(map)
}

#[async_trait]
impl PostRepo for MariaDbPostRepo {
    async fn list(&self, query: ListPostsQuery) -> Result<Vec<Post>, RepoError> {
        let limit = if query.limit == 0 { 50 } else { query.limit as i64 };
        let offset = query.offset as i64;

        let rows: Vec<(PostId, String, String, String, time::OffsetDateTime)> =
            match query.author_id {
                Some(author_id) => sqlx::query_as(
                    "SELECT id, author_id, title, body, created_at \
                     FROM posts WHERE author_id = ? \
                     ORDER BY id DESC LIMIT ? OFFSET ?",
                )
                .bind(author_id.to_string())
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(err)?,
                None => sqlx::query_as(
                    "SELECT id, author_id, title, body, created_at \
                     FROM posts ORDER BY id DESC LIMIT ? OFFSET ?",
                )
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(err)?,
            };

        let ids: Vec<PostId> = rows.iter().map(|r| r.0).collect();
        let mut tag_map = fetch_tags_for(&self.pool, &ids).await?;

        rows.into_iter()
            .map(|(id, author_str, title, body, created_at)| {
                Ok(Post {
                    id,
                    author_id: parse_author(&author_str)?,
                    title,
                    body,
                    tags: tag_map.remove(&id).unwrap_or_default(),
                    created_at,
                })
            })
            .collect()
    }

    async fn get(&self, id: PostId) -> Result<Option<Post>, RepoError> {
        let row: Option<(PostId, String, String, String, time::OffsetDateTime)> = sqlx::query_as(
            "SELECT id, author_id, title, body, created_at FROM posts WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(err)?;

        let Some((id, author_str, title, body, created_at)) = row else {
            return Ok(None);
        };
        let mut tag_map = fetch_tags_for(&self.pool, &[id]).await?;
        Ok(Some(Post {
            id,
            author_id: parse_author(&author_str)?,
            title,
            body,
            tags: tag_map.remove(&id).unwrap_or_default(),
            created_at,
        }))
    }

    async fn create(&self, input: NewPost) -> Result<Post, RepoError> {
        let mut tx = self.pool.begin().await.map_err(err)?;

        let result = sqlx::query("INSERT INTO posts (author_id, title, body) VALUES (?, ?, ?)")
            .bind(input.author_id.to_string())
            .bind(&input.title)
            .bind(&input.body)
            .execute(&mut *tx)
            .await
            .map_err(err)?;
        let id = result.last_insert_id() as PostId;

        let created_at: time::OffsetDateTime =
            sqlx::query_scalar("SELECT created_at FROM posts WHERE id = ?")
                .bind(id)
                .fetch_one(&mut *tx)
                .await
                .map_err(err)?;

        let tag_ids = upsert_tags(&mut tx, &input.tags).await?;
        link_post_tags(&mut tx, id, &tag_ids).await?;

        tx.commit().await.map_err(err)?;

        Ok(Post {
            id,
            author_id: input.author_id,
            title: input.title,
            body: input.body,
            tags: input.tags,
            created_at,
        })
    }

    async fn update(
        &self,
        id: PostId,
        author_id: AuthorId,
        patch: PostPatch,
    ) -> Result<Option<Post>, RepoError> {
        let mut tx = self.pool.begin().await.map_err(err)?;

        let existing: Option<(PostId, String, String, String, time::OffsetDateTime)> =
            sqlx::query_as(
                "SELECT id, author_id, title, body, created_at \
                 FROM posts WHERE id = ? AND author_id = ? FOR UPDATE",
            )
            .bind(id)
            .bind(author_id.to_string())
            .fetch_optional(&mut *tx)
            .await
            .map_err(err)?;

        let Some((_, _, mut title, mut body, created_at)) = existing else {
            return Ok(None);
        };

        if let Some(t) = patch.title {
            title = t;
        }
        if let Some(b) = patch.body {
            body = b;
        }

        sqlx::query("UPDATE posts SET title = ?, body = ? WHERE id = ?")
            .bind(&title)
            .bind(&body)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(err)?;

        let tags = if let Some(new_tags) = patch.tags {
            sqlx::query("DELETE FROM post_tags WHERE post_id = ?")
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(err)?;
            let tag_ids = upsert_tags(&mut tx, &new_tags).await?;
            link_post_tags(&mut tx, id, &tag_ids).await?;
            new_tags
        } else {
            let rows: Vec<(String,)> = sqlx::query_as(
                "SELECT t.name FROM post_tags pt JOIN tags t ON pt.tag_id = t.id \
                 WHERE pt.post_id = ? ORDER BY t.name",
            )
            .bind(id)
            .fetch_all(&mut *tx)
            .await
            .map_err(err)?;
            rows.into_iter().map(|r| r.0).collect()
        };

        tx.commit().await.map_err(err)?;

        Ok(Some(Post {
            id,
            author_id,
            title,
            body,
            tags,
            created_at,
        }))
    }

    async fn delete(&self, id: PostId, author_id: AuthorId) -> Result<bool, RepoError> {
        let result = sqlx::query("DELETE FROM posts WHERE id = ? AND author_id = ?")
            .bind(id)
            .bind(author_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(err)?;
        Ok(result.rows_affected() > 0)
    }
}
