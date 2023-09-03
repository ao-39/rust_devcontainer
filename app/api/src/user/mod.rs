mod register;
use std::sync::Arc;

use app_service::user::IUserApplicationService;
use axum::{routing::post, Extension, Router};
use register::user_register;

pub fn user_router<T>(user_app_service: T) -> Router
where
    T: IUserApplicationService + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(user_register::<T>))
        .layer(Extension(Arc::new(user_app_service)))
}
