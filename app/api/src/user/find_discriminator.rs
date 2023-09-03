use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use app_service::user::{IUserAppService, UserFindError};
use domain::object::{email_address::EmailAddress, url::Url, UserDiscriminator, UserName};
use serde::{Deserialize, Serialize};

pub async fn find_by_discriminator<T>(
    Extension(user_app_service): Extension<Arc<T>>,
    Json(payload): Json<UserFindByDiscriminator>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    T: IUserAppService + Send + Sync + 'static,
{
    let res = user_app_service
        .find_by_discriminator(payload.discriminator)
        .await;

    match res {
        Ok(user) => Ok(Json(UserDto {
            discriminator: user.discriminator,
            name: user.name,
            email: user.email,
            web_page: user.web_page,
        })),
        Err(e) => match e {
            UserFindError::NotFound => Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: "not found",
                }),
            )),
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "internal server error",
                }),
            )),
        },
    }
}

#[derive(Deserialize)]
pub struct UserFindByDiscriminator {
    pub discriminator: UserDiscriminator,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: &'static str,
}

#[derive(Serialize)]
pub struct UserDto {
    discriminator: UserDiscriminator,
    name: UserName,
    email: EmailAddress,
    #[serde(rename = "webPage")]
    web_page: Option<Url>,
}
