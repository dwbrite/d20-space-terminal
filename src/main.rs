use std::io;
use axum::Router;
use axum::routing::get_service;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(
            get_service(ServeFile::new("./target/ui/index.html"))
                .handle_error(|_: io::Error| async move { unimplemented!() }),
        )
        .nest(
            "/static",
            get_service(ServeDir::new("./target/ui"))
                .handle_error(|_: io::Error| async move { unimplemented!() }),
        );

    axum::Server::bind(&"0.0.0.0:42003".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}