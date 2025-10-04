use anyhow::Context;
use chrono::Utc;
use clickhouse::Row;
use serde::Serialize;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Row)]
pub struct ChExcelFile {
    #[serde(with = "clickhouse::serde::uuid")]
    pub file_id: Uuid,
    pub tag: String,
    pub file_name: String,
    pub file_path: String,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub uploaded_at_utc: chrono::DateTime<Utc>,
}

impl ChExcelFile {
    pub(crate) fn new(path: &Path, tag: &str) -> Result<Self, anyhow::Error> {
        let file_path = path.to_str().context("invalid UTF8")?.to_owned();
        let file_name = path
            .file_name()
            .context("error getting filename")?
            .to_str()
            .context("error parsing filename")?
            .to_owned();

        Ok(Self {
            file_id: Uuid::new_v4(),
            tag: tag.to_owned(),
            file_name,
            file_path,
            uploaded_at_utc: Utc::now(),
        })
    }
}
