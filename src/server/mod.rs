use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::out::green_fog;
mod html;

pub async fn start_server(port: impl Into<String>) -> crate::out::Foggy {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/healthz", get(health))
        .route("/", get(html::render_page_html))
        .route("/content", get(html::render_content_html))
        .nest("/", assets());

    let p = format!("0.0.0.0:{}", port.into());
    let listener = tokio::net::TcpListener::bind(p.clone()).await.unwrap();

    let path = format!("http://{}", p);
    if webbrowser::open(&path).is_ok() {
        axum::serve(listener, app).await.unwrap();
    }
    green_fog("Normal server shutdown")
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

use tower_http::services::ServeDir;

fn assets() -> Router {
    Router::new().nest_service("/_assets", using_serve_dir())
}

pub fn using_serve_dir() -> ServeDir {
    ServeDir::new("files/assets")
}
