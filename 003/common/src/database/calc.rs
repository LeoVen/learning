use bson::doc;
use bson::oid::ObjectId;
use bson::Bson;
use bson::Document;
use futures::TryStreamExt;

use crate::model::calc::CalcEntity;

#[derive(Clone)]
pub struct CalcDatabase {
    db_name: String,
    mongo: mongodb::Client,
}

impl CalcDatabase {
    pub fn col() -> &'static str {
        "calc"
    }

    pub fn new(mongo: mongodb::Client, db_name: String) -> Self {
        Self { mongo, db_name }
    }

    pub async fn create(&self, calc: CalcEntity) -> anyhow::Result<ObjectId> {
        let document = bson::to_document(&calc)?;

        let calcs = self.collection();

        let insert_result = calcs.insert_one(document).await?;

        Ok(insert_result
            .inserted_id
            .as_object_id()
            .expect("_id field is not of ObjectId type"))
    }

    pub async fn list(&self, min: i64, max: i64) -> anyhow::Result<Vec<CalcEntity>> {
        let query = doc! {
            "p": doc! {
                "$gte": min,
                "$lte": max,
            }
        };

        let calcs = self.collection();
        let mut result: Vec<CalcEntity> = vec![];
        let mut cursor = calcs.find(query).await?;

        while let Some(doc) = cursor.try_next().await? {
            result.push(bson::from_bson(Bson::Document(doc))?);
        }

        result.sort_by(|a, b| a.p.cmp(&b.p));

        Ok(result)
    }

    pub async fn delete(&self, min: i64, max: i64) -> anyhow::Result<u64> {
        let query = doc! {
            "p": doc! {
                "$gte": min,
                "$lte": max,
            }
        };

        let calcs = self.collection();

        let result = calcs.delete_many(query).await?;

        Ok(result.deleted_count)
    }

    fn collection(&self) -> mongodb::Collection<Document> {
        self.mongo.database(&self.db_name).collection("calc")
    }
}
