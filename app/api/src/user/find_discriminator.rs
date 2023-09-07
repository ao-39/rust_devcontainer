use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use app_service_interface::object::{
    email_address::EmailAddress, url::Url, UserDiscriminator, UserName,
};
use app_service_interface::user::{IUserAppService, UserFindError};
use serde::Serialize;

pub async fn find_by_discriminator<T>(
    Extension(user_app_service): Extension<Arc<T>>,
    Path(user_discriminator): Path<UserDiscriminator>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    T: IUserAppService + Send + Sync + 'static,
{
    let res = user_app_service
        .find_by_discriminator(user_discriminator)
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
