mod config;
mod handlers;
mod routes;
mod middleware;

use routes::{create_routes};

#[tokio::main]
async fn main() {
    let config = config::load_config();

    let addr = format!("0.0.0.0:{}", config.server.port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("\n\nListening on http://{}\n\n", addr);

    let app = create_routes();

    axum::serve(listener, app).await.unwrap();
} 