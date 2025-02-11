use std::time::Duration;

use bson::DateTime;
use common::database::calc::CalcDatabase;
use common::model::calc::CalcEntity;
use tokio::time::Instant;

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

        tokio::spawn(async move {
            let (send, recv) = tokio::sync::oneshot::channel();

            rayon::spawn(move || {
                let _ = send.send(naive_prime_sum_upto_p(p));
            });

            let CalcResult { sum, total, time } = match recv.await {
                Ok(result) => result,
                Err(error) => {
                    let error = error.to_string();
                    tracing::error!(error, "rayon spawn error");
                    return;
                }
            };

            let seconds = time.as_secs();
            tracing::info!(sum, total, p, seconds, "calculated primes");

            let result = service
                .db
                .create(CalcEntity {
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
