use core::str;

use anyhow::Context;
use common::storage::Storage;
use futures::StreamExt;
use lapin::Connection;
use lapin::ConnectionProperties;
use lapin::options::BasicAckOptions;
use lapin::options::BasicConsumeOptions;
use lapin::types::FieldTable;
use serde::Deserialize;

use crate::processor;
use crate::serv_config::QueueConfig;

// Partial payload
#[derive(Deserialize, Debug)]
pub struct EventPayload {
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "Key")]
    pub key: String,
}

#[tracing::instrument(skip_all)]
pub async fn queue_recv(queue_config: &QueueConfig, storage: Storage) -> anyhow::Result<()> {
    let QueueConfig {
        host,
        pass,
        user,
        port,
        consumer,
    } = &queue_config;
    let addr = format!("amqp://{user}:{pass}@{host}:{port}");

    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    tracing::info!("Connected to queue");

    let channel = conn.create_channel().await?;

    let mut consumer = channel
        .basic_consume(
            &consumer.queue_name,
            &consumer.tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(result) = consumer.next().await {
        let delivery = result.context("error receiving delivery")?;

        let data = &delivery.data;
        let payload: EventPayload = serde_json::from_slice(data)?;

        delivery.ack(BasicAckOptions::default()).await?;

        tracing::info!("Message received and acknowledged. Starting processor.");

        let storage = storage.clone();
        tokio::spawn(async move { processor::process(payload, storage).await });
    }

    Ok(())
}
