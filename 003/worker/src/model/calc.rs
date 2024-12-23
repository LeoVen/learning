use std::time::Duration;

use bson::oid::ObjectId;
use bson::DateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Calc {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub p: i64,
    pub created_at: DateTime,
    pub sum: i64,
    pub total: i64,
    pub time: Duration,
}
