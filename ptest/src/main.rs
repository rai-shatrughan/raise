use goose::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(scenario!("TimeSeriesPut").register_transaction(transaction!(ts_put)))
        .execute()
        .await?;

    Ok(())
}

async fn ts_put(user: &mut GooseUser) -> TransactionResult {
    let data = json!([{
        "timestamp": "2022-10-01T08:30:03.780Z",
        "values": [
            {
                "dataPointId": "dp1",
                "value": 985,
                "qualityCode": 0
            },
            {
                "dataPointId": "dp2",
                "value": 374,
                "qualityCode": 0
            }
        ]
    },
    {
        "timestamp": "2022-10-02T08:30:03.780Z",
        "values": [
            {
                "dataPointId": "dp1",
                "value": 987,
                "qualityCode": 0
            },
            {
                "dataPointId": "dp2",
                "value": 376,
                "qualityCode": 0
            }
        ]
    }
    ]);

    let ts_path = "/api/v1/timeseries/6fdae6af-226d-48bd-8b61-699758137eb3";
    let request_builder = user
        .get_request_builder(&GooseMethod::Put, ts_path)?
        .body(data.to_string())
        .header("X-API-Key", "srkey12345")
        .header("Content-Type", "application/json");

    let goose_request = GooseRequest::builder()
        .set_request_builder(request_builder)
        .path(ts_path)
        .method(GooseMethod::Put)
        .build();

    let _goose = user.request(goose_request).await?;

    Ok(())
}
