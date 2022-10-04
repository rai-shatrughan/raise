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
        "timestamp": "2017-07-21T17:32:28Z",
        "property": "temperature",
        "unit": "celcius",
        "value": 80
    },
    {
        "timestamp": "2017-07-21T17:32:28Z",
        "property": "pressure",
        "unit": "ton",
        "value": 20
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
