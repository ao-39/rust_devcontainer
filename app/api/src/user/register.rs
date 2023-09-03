use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use app_service::interface::user::{IUserAppService, UserRegisterError};
use serde::{Deserialize, Serialize};

pub async fn user_register<T>(
    Extension(user_app_service): Extension<Arc<T>>,
    Json(payload): Json<UserRegister>,
) -> Result<StatusCode, impl IntoResponse>
where
    T: IUserAppService + Send + Sync + 'static,
{
    let res = user_app_service
        .register(
            payload.discriminator,
            payload.name,
            payload.email,
            payload.web_page,
        )
        .await;

    match res {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => match e {
            UserRegisterError::DuplicateDiscriminator => Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: "duplicated discriminator",
                }),
            )),
            UserRegisterError::DuplicateEmail => Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: "duplicated email",
                }),
            )),
            UserRegisterError::OtherError => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "internal server error",
                }),
            )),
        },
    }
}

#[derive(Deserialize)]
pub struct UserRegister {
    discriminator: domain::object::UserDiscriminator,
    name: domain::object::UserName,
    email: domain::object::email_address::EmailAddress,
    #[serde(rename = "webPage")]
    web_page: Option<domain::object::url::Url>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: &'static str,
}
