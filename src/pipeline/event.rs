use super::Context;
use serde_json::Value;

#[derive(Clone, Debug)]
pub enum OutputStep {
    Deliver,
    Drop,
}
use utils::nanotime;

#[derive(Clone, Debug)]
pub struct Event {
    pub key: Option<String>,
    pub drop: bool,
    pub raw: String,
    pub json: Option<String>,
    pub parsed: Value,
    pub classification: String,
    pub feedback: Option<f64>,
    pub dimensions: Vec<String>,
    pub index: Option<String>,
    pub data_type: Option<String>,
    pub output_step: OutputStep,
    pub app_epoch_ns: u64,
    pub ingest_time_ns: u64,
    pub ctx: Option<Context>,
}

impl Event {
    pub fn new(raw: &str, ctx: Option<Context>, app_epoch_ns: u64) -> Self {
        let ingest_time_ns = nanotime();
        Event {
            key: None,
            drop: false,
            raw: String::from(raw),
            json: None,
            parsed: Value::Null,
            classification: String::from(""),
            feedback: None,
            ingest_time_ns,
            dimensions: Vec::new(),
            index: None,
            data_type: None,
            output_step: OutputStep::Deliver,
            app_epoch_ns,
            ctx,
        }
    }
    pub fn from(original: Self) -> Self {
        Event {
            key: original.key,
            drop: original.drop,
            raw: original.raw,
            json: original.json,
            parsed: original.parsed,
            classification: original.classification,
            feedback: original.feedback,
            ingest_time_ns: original.ingest_time_ns,
            dimensions: original.dimensions,
            index: original.index,
            data_type: original.data_type,
            output_step: original.output_step,
            app_epoch_ns: original.app_epoch_ns,
            ctx: original.ctx,
        }
    }
}
