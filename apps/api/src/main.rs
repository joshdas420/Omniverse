use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::init();

    // Create our application
    let app = Router::new()
        .route("/", get(|| async { "🚀 OmniVerse API" }))
        .route("/health", get(|| async { "✅ Healthy" }));

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 OmniVerse API server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
