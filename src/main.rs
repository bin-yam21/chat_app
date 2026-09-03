mod db;
mod handlers;
mod auth;
mod middleware;
mod repository;
mod models;
mod routes;

use axum::{
    Router,
    routing::{get, get_service},
    response::{Html, IntoResponse},
    http::header,
};
use axum::http::Method;
// axum::response::IntoResponse is not needed here
use tower_http::cors::{CorsLayer, Any};

use db::init_db;
use routes::create_routes as routes;
use tower_http::services::ServeDir;

// Use tower_http CORS layer (configured below)

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize PostgreSQL connection pool
    let pool = init_db().await;

    // Apply database migrations on startup. Embedding them in the binary means
    // the runtime image needs no sqlx-cli — the server brings its own schema up
    // to date against whatever DATABASE_URL it's pointed at (e.g. Render's
    // managed Postgres).
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    tracing::info!("✅ Database migrations are up to date");

    // Build the app router with all routes and inject DB pool as shared state
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .allow_credentials(false);

    let app: Router = routes(pool)
        // Interactive API documentation (Swagger UI + OpenAPI spec)
        .route("/docs", get(docs_html))
        .route("/openapi.json", get(openapi_spec))
        .nest_service("/uploads", get_service(ServeDir::new("uploads")).handle_error(|error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Static file error: {}", error),
            )
        }))
        .layer(cors);

    // Bind TCP listener. Hosting platforms like Render inject the port to
    // listen on via $PORT, so honour it and fall back to 3000 for local dev.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server running on {}", listener.local_addr().unwrap());

    // Serve the app directly without into_make_service()
    axum::serve(listener, app).await.unwrap();
}

/// Swagger UI page (loads the OpenAPI spec from `/openapi.json`).
async fn docs_html() -> Html<&'static str> {
    Html(include_str!("../docs.html"))
}

/// The OpenAPI 3.0 specification describing the whole API.
async fn openapi_spec() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/json")],
        include_str!("../openapi.json"),
    )
}
