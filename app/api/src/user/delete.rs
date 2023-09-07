use std::sync::Arc;

use app_service_interface::{object::UserDiscriminator, user::IUserAppService};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};

pub async fn user_delete<T>(
    Extension(user_app_service): Extension<Arc<T>>,
    Path(user_discriminator): Path<UserDiscriminator>,
) -> Result<StatusCode, impl IntoResponse>
where
    T: IUserAppService + Send + Sync + 'static,
{
    let res = user_app_service.delete(user_discriminator).await;
    match res {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
