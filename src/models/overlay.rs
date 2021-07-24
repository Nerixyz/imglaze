use errors::sql::SqlResult;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Serialize)]
pub struct Overlay {
    pub id: i32,
    pub created_by: String,
    pub for_user: String,
    pub secret: String,
    #[serde(skip)]
    pub last_image: Option<String>,
}

impl Overlay {
    pub async fn patch_secret(&self, pool: &PgPool) -> SqlResult<()> {
        // language=PostgreSQL
        sqlx::query!(
            "UPDATE overlays SET secret = $2 WHERE id = $1",
            self.id,
            self.secret
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn patch_image(&self, pool: &PgPool) -> SqlResult<()> {
        // language=PostgreSQL
        sqlx::query!(
            "UPDATE overlays SET last_image = $2 WHERE id = $1",
            self.id,
            self.last_image
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, pool: &PgPool) -> SqlResult<()> {
        // language=PostgreSQL
        sqlx::query!("DELETE FROM overlays WHERE id = $1", self.id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

pub async fn by_id(id: i32, pool: &PgPool) -> SqlResult<Overlay> {
    // language=PostgreSQL
    let overlay = sqlx::query_as!(Overlay, "SELECT * FROM overlays WHERE id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(overlay)
}

pub async fn all_from_user(user_id: &str, pool: &PgPool) -> SqlResult<Vec<Overlay>> {
    // language=PostgreSQL
    let overlays = sqlx::query_as!(
        Overlay,
        "SELECT * FROM overlays WHERE created_by = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;
    Ok(overlays)
}

pub async fn by_login(channel_login: &str, pool: &PgPool) -> SqlResult<Overlay> {
    // language=PostgreSQL
    let overlay = sqlx::query_as!(
        Overlay,
        "SELECT * FROM overlays WHERE for_user = $1",
        channel_login
    )
    .fetch_one(pool)
    .await?;
    Ok(overlay)
}

pub async fn creator_for(channel_login: &str, pool: &PgPool) -> SqlResult<String> {
    // language=PostgreSQL
    let creator = sqlx::query_scalar!(r#"SELECT u.name FROM overlays LEFT JOIN users u on u.id = overlays.created_by WHERE for_user = $1"#, channel_login).fetch_one(pool).await?;
    Ok(creator)
}

pub async fn create(
    user_id: &str,
    for_user: &str,
    secret: &str,
    pool: &PgPool,
) -> SqlResult<Overlay> {
    // language=PostgreSQL
    let overlay = sqlx::query_as!(Overlay, "INSERT INTO overlays (created_by, for_user, secret) VALUES ($1, $2, $3) RETURNING id, created_by, for_user, secret, last_image", user_id, for_user, secret).fetch_one(pool).await?;
    Ok(overlay)
}

pub async fn all_channels(pool: &PgPool) -> SqlResult<Vec<String>> {
    // language=PostgreSQL
    let channels = sqlx::query_scalar!("SELECT DISTINCT for_user FROM overlays")
        .fetch_all(pool)
        .await?;
    Ok(channels)
}
