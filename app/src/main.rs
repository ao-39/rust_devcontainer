mod common;

use axum::{routing::get, Router};

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(OpenApi)]
    #[openapi(
        paths(health),
        tags(
            (name = "APIドキュメント", description = "APIドキュメントです。")
        )
    )]
    struct ApiDoc;

    dotenvy::dotenv()?;
    common::logger::init();
    tracing::info!("アプリケーションが起動しました");
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .route("/helth", get(health));

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/helth",
    responses(
        (status = 200, description = "ヘルスチェック")
    )
)]
async fn health() -> &'static str {
    "OK"
}
