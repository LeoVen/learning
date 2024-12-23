use bson::doc;
use bson::oid::ObjectId;
use mongodb::Client;

use crate::config::AppConfig;
use crate::model::calc::Calc;

#[derive(Clone)]
pub struct CalcDatabase {
    db_name: String,
    mongo: Client,
}

impl CalcDatabase {
    pub fn col() -> &'static str {
        "calc"
    }

    pub fn new(config: &AppConfig, mongo: Client) -> Self {
        // TODO set database and collections for calc
        Self {
            mongo,
            db_name: config.database_name.clone(),
        }
    }

    pub async fn create(&self, calc: Calc) -> anyhow::Result<ObjectId> {
        let document = doc! {
            "p": calc.p,
            "created_at": calc.created_at,
            "sum": calc.sum,
            "total": calc.total,
            "time": calc.time.as_secs() as i64,
        };

        let calcs = self
            .mongo
            .database(&self.db_name)
            .collection(CalcDatabase::col());

        let insert_result = calcs.insert_one(document).await?;

        Ok(insert_result
            .inserted_id
            .as_object_id()
            .expect("_id field is not of ObjectId type"))
    }

    // Create, Read, Update, Delete
}
