use crate::model::timeseries::Timeseries;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::json;
use std::time::Duration;

pub async fn ts_put(
    Extension(kafka_producer): Extension<FutureProducer>,
    Path(asset_id): Path<String>,
    Json(payload): Json<Vec<Timeseries>>,
) -> impl IntoResponse {
    // let payload_string = format!("{:?}", &payload);

    let payload_string = match serde_json::to_string(&payload){
        Ok(payload_string) => payload_string,
        Err(_e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "TS": "FAILED" })))
    };

    kafka_producer
    .send(
        FutureRecord::to("ts")
            .key(&asset_id)
            .payload(&payload_string),
        Duration::from_secs(0),
    )
    .await
    .expect("Failed to put msg into Kafka");


    (StatusCode::OK, Json(json!({ "TS": "OK" })))
}
