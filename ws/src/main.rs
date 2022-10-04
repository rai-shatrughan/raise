use axum::{
    routing::{get, put, get_service},
    extract::Extension,
    Router,
    http::StatusCode,
    response::IntoResponse,

};
use std::{io};
use tower_http::{trace::TraceLayer};
use utils::conf::Props;

extern crate utils;

mod handler;
mod model;

#[tokio::main]
async fn main() {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "example_tracing_aka_logging=error,tower_http=error",
        )
    }

    tracing_subscriber::fmt::init();
    let kafka_producer = utils::KafkaProducer::init();
    let service = tower_http::services::ServeDir::new(Props::init().web_dir);
    let app = Router::new()
                .route("/api/v1/am", get(handler::asset::hello))
                .route("/api/v1/timeseries/:asset_id", put(handler::timeseries::ts_put))
                .layer(Extension(kafka_producer))
                .layer(TraceLayer::new_for_http())
                .fallback(get_service(service).handle_error(handle_error));

    println!("*** serving web_dir {} - at address {}", Props::init().web_dir, Props::init().web_address);

    axum::Server::bind(&Props::init().web_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Error Serving Static Server")
}