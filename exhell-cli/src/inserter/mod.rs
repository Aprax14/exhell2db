use exhell_utils::{
    FileContent,
    excel::{cell::ChExcelCell, file::ChExcelFile, sheet::ChExcelSheet},
};

#[derive(Debug)]
pub struct FileInserterBatch {
    files: Vec<ChExcelFile>,
    sheets: Vec<ChExcelSheet>,
    cells: Vec<ChExcelCell>,
    batch_size: usize,
    current_batch_size: usize,
}

impl Default for FileInserterBatch {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            sheets: Vec::new(),
            cells: Vec::with_capacity(50_000),
            batch_size: 50_000,
            current_batch_size: 0,
        }
    }
}

impl FileInserterBatch {
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size,
            cells: Vec::with_capacity(batch_size),
            ..Default::default()
        }
    }

    pub async fn push(
        &mut self,
        mut file_content: FileContent,
        client: &clickhouse::Client,
    ) -> anyhow::Result<()> {
        self.files.push(file_content.excel);
        self.sheets.append(&mut file_content.sheets);
        self.cells.append(&mut file_content.cells);
        self.current_batch_size += file_content.cells.len();

        if self.current_batch_size >= self.batch_size {
            self.insert(client).await?;
        }

        Ok(())
    }

    fn clear(&mut self) {
        self.files.clear();
        self.sheets.clear();
        self.cells.clear();
        self.current_batch_size = 0;
    }

    async fn insert(&mut self, client: &clickhouse::Client) -> anyhow::Result<()> {
        let mut inserter = client.inserter::<ChExcelFile>("files");
        for file in &self.files {
            inserter.write(file).await?;
        }
        inserter.end().await?;

        let mut inserter = client.inserter::<ChExcelSheet>("excel_sheets");
        for sheet in &self.sheets {
            inserter.write(sheet).await?;
        }
        inserter.end().await?;

        let mut inserter = client.inserter::<ChExcelCell>("excel_cells");
        for cell in &self.cells {
            inserter.write(cell).await?;
        }
        inserter.end().await?;

        self.clear();

        Ok(())
    }

    pub async fn flush(&mut self, client: &clickhouse::Client) -> anyhow::Result<()> {
        self.insert(client).await
    }
}
