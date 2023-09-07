use std::sync::Arc;

use app_service_interface::object::email_address::EmailAddress;
use app_service_interface::object::url::Url;
use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use app_service_interface::object::{UserDiscriminator, UserName};
use app_service_interface::user::{IUserAppService, UserUpdateOperator};
use serde::Deserialize;

pub async fn user_update<T>(
    Extension(user_app_service): Extension<Arc<T>>,
    Path(user_discriminator): Path<UserDiscriminator>,
    Json(payload): Json<UserUpdateOperatorReqBody>,
) -> Result<StatusCode, impl IntoResponse>
where
    T: IUserAppService + Send + Sync + 'static,
{
    let res = user_app_service
        .update(user_discriminator, payload.into())
        .await;
    match res {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserUpdateOperatorReqBody {
    Discriminator(UserDiscriminator),
    Name(UserName),
    Email(EmailAddress),
    WebPage(Option<Url>),
}

impl From<UserUpdateOperatorReqBody> for UserUpdateOperator {
    fn from(req_body: UserUpdateOperatorReqBody) -> Self {
        match req_body {
            UserUpdateOperatorReqBody::Discriminator(discriminator) => {
                UserUpdateOperator::Discriminator(discriminator)
            }
            UserUpdateOperatorReqBody::Name(name) => UserUpdateOperator::Name(name),
            UserUpdateOperatorReqBody::Email(email) => UserUpdateOperator::Email(email),
            UserUpdateOperatorReqBody::WebPage(web_page) => UserUpdateOperator::WebPage(web_page),
        }
    }
}
