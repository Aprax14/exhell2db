use std::path::Path;

use calamine::{Reader, open_workbook_auto};

use crate::excel::{cell::ChExcelCell, file::ChExcelFile, sheet::ChExcelSheet};

pub mod excel;

#[derive(Debug, Clone)]
pub struct FileContent {
    pub excel: ChExcelFile,
    pub sheets: Vec<ChExcelSheet>,
    pub cells: Vec<ChExcelCell>,
}

impl FileContent {
    pub fn from_path(path: &Path, set_tag: &str) -> Result<Self, anyhow::Error> {
        let mut excel_content = open_workbook_auto(path)?;
        let sheet_names = excel_content.sheet_names();

        let excel = ChExcelFile::new(path, set_tag)?;
        let mut sheets = Vec::with_capacity(sheet_names.len());
        let mut cells = Vec::new();
        for sheet_name in &sheet_names {
            let sheet = ChExcelSheet::new(excel.file_id, sheet_name);
            let sheet_content = excel_content.worksheet_range(sheet_name)?;
            let sheet_cells = ChExcelCell::extract_from_sheet(sheet.sheet_id, &sheet_content)?;
            sheets.push(sheet);
            cells.extend(sheet_cells);
        }

        Ok(Self {
            excel,
            sheets,
            cells,
        })
    }
}
