use clickhouse::Row;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Row)]
pub struct ChExcelSheet {
    #[serde(with = "clickhouse::serde::uuid")]
    pub file_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub sheet_id: Uuid,
    pub sheet_name: String,
}

impl ChExcelSheet {
    pub(crate) fn new(file_id: Uuid, sheet_name: &str) -> Self {
        Self {
            file_id,
            sheet_id: Uuid::new_v4(),
            sheet_name: sheet_name.to_owned(),
        }
    }
}
