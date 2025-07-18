use std::io::Cursor;

use anyhow::Context;
use common::storage::Storage;
use image::{GenericImageView, Pixel};

use crate::queue::EventPayload;

#[tracing::instrument(skip(storage))]
pub async fn process(event: EventPayload, storage: Storage) {
    let EventPayload { event_name, key } = event;

    if event_name != "s3:ObjectCreated:Put" {
        tracing::error!("Received unexpected event: {}. Aborting.", event_name);
        return;
    }

    match process_file(&key, storage).await {
        Ok(_) => {}
        Err(err) => {
            let err = err.to_string();
            tracing::error!(err, "File processing error");
        }
    }
}

#[tracing::instrument(skip(storage))]
async fn process_file(key: &str, storage: Storage) -> anyhow::Result<()> {
    tracing::info!("Started processing.");

    let mut splits = key.split('/');
    let bucket = splits.next().context("expected bucket name in split")?;
    let key = splits.fold(String::new(), |acc, item| acc + item);

    let stream = storage.stream_file(bucket, &key).await?;
    let raw_data = stream.collect().await?.to_vec();

    let image = image::ImageReader::new(Cursor::new(raw_data))
        .with_guessed_format()?
        .decode()?;

    let mut avg: [u64; 3] = [0, 0, 0];
    let mut len = 0;
    for (_, _, rgba) in image.pixels().into_iter() {
        let rgb = rgba.to_rgb();
        let chann = rgb.channels();

        avg[0] += chann[0] as u64;
        avg[1] += chann[1] as u64;
        avg[2] += chann[2] as u64;
        len += 1;
    }

    let avg = [avg[0] / len, avg[1] / len, avg[2] / len];
    tracing::info!(avg = format!("{:?}", avg), "Calculation Results");

    Ok(())
}
