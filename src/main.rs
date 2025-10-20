mod db;
mod handlers;
mod repository;
mod models;
mod routes;

use axum::Router;
use db::init_db;
use routes::create_routes as routes;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize PostgreSQL connection pool
    let pool = init_db().await;

    // Build the app router with all routes and inject DB pool as shared state
    let app: Router = routes(pool);

    // Bind TCP listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server running on {}", listener.local_addr().unwrap());

    // Serve the app directly without into_make_service()
    axum::serve(listener, app).await.unwrap();
}
