mod delete;
mod find_discriminator;
mod register;
mod update;
use std::sync::Arc;

use app_service_interface::user::IUserAppService;
use axum::{
    routing::{delete, get, patch, post},
    Extension, Router,
};
use delete::user_delete;
use register::user_register;
use update::user_update;

use self::find_discriminator::find_by_discriminator;

pub fn user_router<T>(user_app_service: T) -> Router
where
    T: IUserAppService + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(user_register::<T>))
        .route("/:user_discriminator", get(find_by_discriminator::<T>))
        .route("/:user_discriminator", delete(user_delete::<T>))
        .route("/:user_discriminator", patch(user_update::<T>))
        .layer(Extension(Arc::new(user_app_service)))
}
