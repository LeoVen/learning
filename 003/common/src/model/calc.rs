use std::time::Duration;

use bson::oid::ObjectId;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::DurationMilliSeconds;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct CalcEntity {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub p: i64,
    pub created_at: bson::DateTime,
    pub sum: i64,
    pub total: i64,
    #[serde_as(as = "DurationMilliSeconds")]
    pub time: Duration,
}

#[derive(Serialize, Deserialize)]
pub struct Calc {
    pub id: Option<String>,
    pub p: i64,
    pub created_at: String,
    pub sum: i64,
    pub total: i64,
    pub time: Duration,
}

impl From<CalcEntity> for Calc {
    fn from(value: CalcEntity) -> Self {
        Calc {
            id: value.id.map(|object_id| object_id.to_string()),
            p: value.p,
            created_at: value.created_at.to_chrono().to_rfc3339(),
            sum: value.sum,
            total: value.total,
            time: value.time,
        }
    }
}
