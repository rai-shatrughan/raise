use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Timeseries {
    timestamp: String,
    property: String,
    unit: String,
    value: f64,
}

// this is not needed with `serde_json::to_string(&payload).unwrap()`
// look at this as a reference for future
impl fmt::Display for Timeseries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
