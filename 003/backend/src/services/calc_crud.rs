use common::database::calc::CalcDatabase;
use common::model::calc::Calc;

#[derive(Clone)]
pub struct CalcCrudService {
    db: CalcDatabase,
}

impl CalcCrudService {
    pub fn new(db: CalcDatabase) -> Self {
        Self { db }
    }

    pub async fn list_results(
        &self,
        min: Option<i64>,
        max: Option<i64>,
    ) -> anyhow::Result<Vec<Calc>> {
        let min = min.unwrap_or(0);
        let max = max.unwrap_or(i64::MAX);

        Ok(self
            .db
            .list(min, max)
            .await?
            .into_iter()
            .map(From::from)
            .collect())
    }

    pub async fn delete_results(&self, min: Option<i64>, max: Option<i64>) -> anyhow::Result<u64> {
        let min = min.unwrap_or(0);
        let max = max.unwrap_or(i64::MAX);

        self.db.delete(min, max).await
    }
}
