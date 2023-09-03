use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use tracing::info;
mod user;
use app_service::user::IUserApplicationService;

pub async fn run<T>(user_app_service: T) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T: IUserApplicationService + Send + Sync + 'static,
{
    let user_router = Router::new()
        .route("/", post(user::user_register::<T>))
        .layer(Extension(Arc::new(user_app_service)));

    let app = Router::new()
        .route("/helth", get(helth))
        .nest("/user", user_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn helth() -> &'static str {
    "ok"
}
