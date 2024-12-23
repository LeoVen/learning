use std::time::Duration;

use bson::DateTime;
use tokio::time::Instant;

use crate::database::calc::CalcDatabase;
use crate::model::calc::Calc;

#[derive(Clone)]
pub struct CalcService {
    db: CalcDatabase,
}

impl CalcService {
    pub fn new(db: CalcDatabase) -> Self {
        Self { db }
    }

    pub fn calculate_prime(&self, p: i64) {
        let service = self.clone();
        // TODO increment load with p

        tokio::spawn(async move {
            let CalcResult { sum, total, time } = naive_prime_sum_upto_p(p);
            let seconds = time.as_secs();
            tracing::info!(sum, total, p, seconds, "calculated primes");

            let result = service
                .db
                .create(Calc {
                    id: None,
                    p,
                    created_at: DateTime::now(),
                    sum,
                    total,
                    time,
                })
                .await;

            match result {
                Ok(id) => {
                    let id = id.to_string();
                    tracing::info!(id, "Created document");
                }
                Err(err) => {
                    let err = err.to_string();
                    tracing::error!(err, "Error when creating Calc document")
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct CalcResult {
    pub sum: i64,
    pub total: i64,
    pub time: Duration,
}

fn naive_prime_sum_upto_p(p: i64) -> CalcResult {
    let before = Instant::now();

    let mut total: i64 = 0;
    let mut sum = 0;

    for n in 3..=p {
        let mut is_prime = true;

        for i in 2..n {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            sum += n;
            total += 1;
        }
    }

    let after = Instant::now();
    let time = after - before;

    CalcResult { sum, total, time }
}
