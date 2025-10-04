use anyhow::ensure;
use calamine::{Data, Range};
use clickhouse::Row;
use serde::Serialize;
use serde_repr::Serialize_repr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr)]
#[repr(i8)]
pub enum CellType {
    String = 1,
    Number = 2,
    Boolean = 3,
    Empty = 4,
    DateTime = 5,
    Error = 6,
}

impl From<&Data> for CellType {
    fn from(value: &Data) -> Self {
        match value {
            Data::Int(_) | Data::Float(_) => Self::Number,
            Data::Bool(_) => Self::Boolean,
            Data::DateTime(_) | Data::DateTimeIso(_) | Data::DurationIso(_) => Self::DateTime,
            Data::Error(_) => Self::Error,
            Data::String(_) => Self::String,
            Data::Empty => Self::Empty,
        }
    }
}

impl CellType {
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
}

#[derive(Debug, Clone, Serialize, Row)]
pub struct ChExcelCell {
    #[serde(with = "clickhouse::serde::uuid")]
    pub cell_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub sheet_id: Uuid,
    pub col_index: u32,
    pub row_index: u32,
    pub cell_type: CellType,
    pub value: String,
}

impl ChExcelCell {
    pub(crate) fn extract_from_sheet(
        sheet_id: Uuid,
        sheet: &Range<Data>,
    ) -> Result<Vec<Self>, anyhow::Error> {
        let (start_row, start_col) = sheet.start().unwrap_or_default();
        let (end_row, end_col) = sheet.end().unwrap_or_default();

        ensure!(
            end_row >= start_row && end_col >= start_col,
            "Range error: ending cell seems to be smaller than the starting cell"
        );

        let expected_size = (end_row - start_row) * (end_col - start_col);

        let mut cells_data: Vec<ChExcelCell> = Vec::with_capacity(expected_size as usize);
        for row_index in start_row..end_row + 1 {
            for col_index in start_col..end_col + 1 {
                if let Some(val) = sheet.get_value((row_index, col_index)) {
                    let cell_type: CellType = val.into();
                    if cell_type.is_empty() {
                        continue;
                    }
                    let value = val.to_string();

                    cells_data.push(ChExcelCell {
                        cell_id: Uuid::new_v4(),
                        sheet_id,
                        col_index,
                        row_index,
                        cell_type,
                        value,
                    });
                }
            }
        }

        Ok(cells_data)
    }
}
