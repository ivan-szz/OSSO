use crate::app::AppState;
use crate::repository::user;
use crate::repository::user::{PublicUser, User};
use crate::schema::user::UserSchema;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("user not found")]
    UserNotFound,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error(transparent)]
    UserRepository(#[from] user::Error),
    #[error(transparent)]
    ValidationErrors(#[from] validator::ValidationErrors),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::UserRepository(e) => {
                tracing::error!(err = %e, "User repository error:");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Internal Server Error"})),
                )
            }
            AuthError::ValidationErrors(e) => {
                let details = e
                    .field_errors()
                    .keys()
                    .map(|key| match key.as_ref() {
                        "email" => "Invalid email format",
                        "password" => "Invalid password format: it must be 8 to 32 characters long",
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
            AuthError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                Json(json!({"message": "User already exists"})),
            ),
            AuthError::UserNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "User not found" })),
            ),
        }
        .into_response()
    }
}

pub async fn register(
    state: State<AppState>,
    Json(user_schema): Json<UserSchema>,
) -> Result<impl IntoResponse, AuthError> {
    user_schema.validate()?;
    let existing_user = User::find_by_email(&user_schema.email, &state.db).await?;

    if existing_user.is_some() {
        return Err(AuthError::UserAlreadyExists);
    }

    let user = User::create(&user_schema, &state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "user": PublicUser::from(&user) })),
    ))
}

pub async fn login(
    state: State<AppState>,
    Json(user_schema): Json<UserSchema>,
) -> Result<impl IntoResponse, AuthError> {
    let user = user::User::find_by_email(&user_schema.email, &state.db).await?;
    let user = user.ok_or(AuthError::InvalidCredentials)?;
    user.verify_password(&user_schema.password)
        .map_err(|_| AuthError::InvalidCredentials)?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Login successful" })),
    ))
}
