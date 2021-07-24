use crate::constants::{TWITCH_CLIENT_ID, TWITCH_CLIENT_SECRET};
use errors::sql::SqlResult;
use sqlx::{FromRow, PgPool};
use std::time::Duration;
use twitch_api2::twitch_oauth2::{AccessToken, ClientId, ClientSecret, RefreshToken, UserToken};

#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub access_token: String,
    pub refresh_token: String,
    pub scopes: String,
}

impl User {
    pub async fn by_id(id: &str, pool: &PgPool) -> SqlResult<User> {
        // language=PostgreSQL
        let user: User = sqlx::query_as!(
            User,
            "SELECT id, access_token, refresh_token, scopes, name FROM users WHERE id = $1",
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn create(&self, pool: &PgPool) -> SqlResult<()> {
        let mut tx = pool.begin().await?;
        // language=PostgreSQL
        let _ = sqlx::query!(
            r#"
            INSERT
            INTO users (id, access_token, refresh_token, scopes, name)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT(id)
                DO UPDATE SET access_token= $2, refresh_token=$3
                "#,
            self.id,
            self.access_token,
            self.refresh_token,
            self.scopes,
            self.name
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

impl From<User> for UserToken {
    fn from(u: User) -> Self {
        Self::from_existing_unchecked(
            AccessToken::new(u.access_token),
            RefreshToken::new(u.refresh_token),
            ClientId::new(TWITCH_CLIENT_ID.to_string()),
            ClientSecret::new(TWITCH_CLIENT_SECRET.to_string()),
            u.name,
            u.id,
            None,
            // this isn't used anywhere
            Some(Duration::from_secs(1000)),
        )
    }
}

pub async fn all(pool: &PgPool) -> SqlResult<Vec<User>> {
    // language=PostgreSQL
    let users = sqlx::query_as!(
        User,
        "SELECT id, access_token, refresh_token, scopes, name FROM users"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn delete(id: &str, pool: &PgPool) -> SqlResult<()> {
    let mut tx = pool.begin().await?;
    // language=PostgreSQL
    let _ = sqlx::query!(
        r#"
            DELETE FROM users WHERE id = $1
                "#,
        id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn update_tokens(
    id: &str,
    access_token: &str,
    refresh_token: &str,
    pool: &PgPool,
) -> SqlResult<()> {
    let mut tx = pool.begin().await?;
    // language=PostgreSQL
    let _ = sqlx::query!(
        r#"
            UPDATE users
            SET access_token = $2, refresh_token = $3
            WHERE id = $1
            "#,
        id,
        access_token,
        refresh_token
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
