use crate::repository::user;
use crate::repository::user::{PublicUser, User};
use crate::schema::auth::RefreshTokenSchema;
use crate::schema::user::UserSchema;
use crate::utils::jwt;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use headers::Authorization;
use redis::AsyncTypedCommands;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("user not found")]
    UserNotFound,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
    #[error(transparent)]
    UserRepository(#[from] user::Error),
    #[error(transparent)]
    ValidationErrors(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::ValidationErrors(e) => {
                let details = e
                    .field_errors()
                    .keys()
                    .map(|key| match key.as_ref() {
                        "email" => "Invalid email format",
                        "password" => "Invalid password format: it must be 8 to 32 characters long",
                        "refresh_token" => "Invalid refresh token format",
                        _ => "Invalid field",
                    })
                    .collect::<Vec<_>>();
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Validation Failed",
                        "details": details
                    })),
                )
            }
            AuthError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid credentials"})),
            ),
            AuthError::InvalidRefreshToken => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Invalid refresh token"})),
            ),
            AuthError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                Json(json!({"message": "User already exists"})),
            ),
            AuthError::UserNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "User not found" })),
            ),
            err => {
                tracing::error!(err = %err, "Internal Server Error:");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Internal Server Error"})),
                )
            }
        }
        .into_response()
    }
}

fn get_refresh_token_expiration() -> u64 {
    7 * 24 * 60 * 60
}

pub async fn register(
    state: State<AppState>,
    Json(user_schema): Json<UserSchema>,
) -> Result<impl IntoResponse, AuthError> {
    user_schema.validate()?;
    let AppState { db, .. } = state.0;
    let existing_user = User::find_by_email(&user_schema.email, &db).await?;

    if existing_user.is_some() {
        return Err(AuthError::UserAlreadyExists);
    }

    let user = User::create(&user_schema, &db).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "user": PublicUser::from(&user) })),
    ))
}

pub async fn login(
    state: State<AppState>,
    Json(user_schema): Json<UserSchema>,
) -> Result<impl IntoResponse, AuthError> {
    let AppState { db, redis, .. } = state.0;
    let mut con = redis.get_multiplexed_async_connection().await?;

    let user = User::find_by_email(&user_schema.email, &db)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;
    user.verify_password(&user_schema.password)
        .map_err(|_| AuthError::InvalidCredentials)?;

    let jwt = jwt::generate(user.id, &user.email)?;
    let refresh_token = jwt::generate_refresh_token();
    let expiration = get_refresh_token_expiration();

    con.set_ex(
        format!("osso:refresh_token:{}", refresh_token),
        user.id.to_string(),
        expiration,
    )
    .await?;

    Ok((
        StatusCode::OK,
        Json(json!({ "access_token": jwt, "refresh_token": refresh_token })),
    ))
}

pub async fn logout(
    state: State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(refresh_token_schema): Json<RefreshTokenSchema>,
) -> Result<impl IntoResponse, AuthError> {
    refresh_token_schema.validate()?;
    let AppState { redis, .. } = state.0;
    let RefreshTokenSchema { refresh_token } = refresh_token_schema;
    let redis_key = format!("osso:refresh_token:{}", refresh_token);
    let mut con = redis.get_multiplexed_async_connection().await?;

    let token = auth.token();
    let claims = jwt::verify(token)?;

    let token_record = con
        .get(&redis_key)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    if token_record != claims.sub.to_string() {
        return Err(AuthError::InvalidCredentials);
    };

    // TODO: Add a blacklist of logged out tokens using a jti

    // TODO: Use redis transaction to avoid race conditions
    con.del(&redis_key).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn refresh(
    state: State<AppState>,
    Json(refresh_token_schema): Json<RefreshTokenSchema>,
) -> Result<impl IntoResponse, AuthError> {
    refresh_token_schema.validate()?;
    let RefreshTokenSchema { refresh_token } = refresh_token_schema;
    let AppState { redis, db, .. } = state.0;
    let mut con = redis.get_multiplexed_async_connection().await?;

    // Get user from redis token
    let token_record = con
        .get_del(format!("osso:refresh_token:{}", refresh_token))
        .await?
        .ok_or(AuthError::InvalidRefreshToken)?;
    let uuid = Uuid::parse_str(&token_record)?;
    let user = User::find_by_id(uuid, &db)
        .await?
        .ok_or(AuthError::InvalidRefreshToken)?;

    // Generate new tokens
    let jwt = jwt::generate(user.id, &user.email)?;
    let refresh_token = jwt::generate_refresh_token();
    let expiration = get_refresh_token_expiration();

    con.set_ex(
        format!("osso:refresh_token:{}", refresh_token),
        user.id.to_string(),
        expiration,
    )
    .await?;

    Ok((
        StatusCode::OK,
        Json(json!({ "access_token": jwt, "refresh_token": refresh_token })),
    ))
}
