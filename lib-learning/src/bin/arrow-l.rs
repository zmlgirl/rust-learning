use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Result;

use arrow::{
    array::{
        Float32Builder, StringBuilder,
        TimestampMillisecondBuilder, 
    },
    datatypes::{DataType, Field, Schema},
    ipc::writer::StreamWriter,
    record_batch::RecordBatch,
};

#[tokio::main]
async fn main() -> Result<()> {
    let stream = std::net::TcpStream::connect("127.0.0.1:6051")?;

    let mut metadata = HashMap::new();
    metadata.insert(String::from("version"), String::from("1.0"));
    metadata.insert(String::from("stream"), String::from("point"));
    metadata.insert(String::from("ack"), String::from("none"));
    let flat_columns = vec![
        Field::new(
            "ts",
            DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None),
            false,
        ),
        Field::new("id", DataType::Utf8, false),
        Field::new("value", DataType::Float32, false),
    ];
    let record_list = DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None);

    let schema = Schema::new(flat_columns).with_metadata(metadata);
    let mut writer = StreamWriter::try_new(&stream, &schema)?;
    let mut records = 0;
    let now = chrono::Utc::now();
    let mut ms = now.timestamp_millis() - 10000;

    loop {
        let mut timestamp_builder = TimestampMillisecondBuilder::new();
        timestamp_builder.append_value(ms + 1000);
        let timestamp = timestamp_builder.finish();
        let mut id_builder = StringBuilder::new();
        id_builder.append_value("ns=2;s=testnode/shyk/FCS0104!122-TI-40413.PV");
        let id = id_builder.finish();
        let mut value_builder = Float32Builder::new();
        value_builder.append_value(127.3);
        let value = value_builder.finish();
        let batch = RecordBatch::try_new(
            Arc::new(schema.clone()),
            vec![
                Arc::new(timestamp),
                Arc::new(id),
                Arc::new(value),
            ],
        )?;
        dbg!(&batch);
        writer.write(&batch)?;
        let mut timestamp_builder = TimestampMillisecondBuilder::new();
        timestamp_builder.append_value(ms + 1000);
        let timestamp = timestamp_builder.finish();
        let mut id_builder = StringBuilder::new();
        id_builder.append_value("ns=2;s=testnode/shyk/FCS0104!122-TI-11111.PV");
        let id = id_builder.finish();
        let mut value_builder = Float32Builder::new();
        value_builder.append_value(127.3);
        let value = value_builder.finish();
        let batch = RecordBatch::try_new(
            Arc::new(schema.clone()),
            vec![
                Arc::new(timestamp),
                Arc::new(id),
                Arc::new(value),
            ],
        )?;
        dbg!(&batch);
        writer.write(&batch)?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}