use crate::schema::user::UserSchema;
use crate::utils::argon::{hash, verify};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub email: String,
}

impl From<&User> for PublicUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
        }
    }
}

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("hashing error: {0}")]
    Hashing(#[from] argon2::password_hash::Error),
}

impl User {
    pub async fn create(value: &UserSchema, pool: &PgPool) -> Result<Self, Error> {
        let UserSchema { email, password } = value;

        let hashed_password = hash(password)?;

        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, password_hash, created_at as \"created_at!\", updated_at",
            email,
            hashed_password
        )
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as!(User, "SELECT id, email, password_hash, created_at as \"created_at!\", updated_at FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as!(User, "SELECT id, email, password_hash, created_at as \"created_at!\", updated_at FROM users WHERE id = $1", id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub fn verify_password(&self, password: &str) -> Result<(), Error> {
        verify(password, &self.password_hash)?;
        Ok(())
    }
}
