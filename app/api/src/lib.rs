use std::net::SocketAddr;

use axum::{routing::get, Router};
use tracing::info;
mod user;
use app_service_interface::user::IUserAppService;

pub async fn run(
    addr: SocketAddr,
    user_app_service: impl IUserAppService + Send + Sync + 'static,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_router = user::user_router(user_app_service);

    let app = Router::new()
        .route("/helth", get(helth))
        .nest("/user", user_router);

    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn helth() -> &'static str {
    "ok"
}
