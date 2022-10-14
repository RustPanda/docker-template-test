use std::collections::BTreeMap;

use axum::{routing::get, Router};
use tokio::signal;

#[tokio::main]
async fn main() {
    let tcp_listener = std::net::TcpListener::bind("0.0.0.0:8080").unwrap();

    let app = Router::new().route(
        "/",
        get(|| async {
            json::stringify_pretty(std::env::vars().collect::<BTreeMap<String, String>>(), 2)
        }),
    );

    axum::Server::from_tcp(tcp_listener)
        .unwrap()
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
