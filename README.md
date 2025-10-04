# Exhell to ClickHouse

I wrote this utility to convert large amounts of Excel files with a similar structure into a ClickHouse database. This allows querying specific cells efficiently and extracting data from all the files.

For companies that use Excel as one of their primary tools, it often happens that the same template is used for many customers. Over time, you may realize that all these spreadsheets contain a lot of useful data that you would like to analyze, filter, and manipulate. Doing this manually on each file would take ages. 
I found myself too many times in this situation, so i created this simple tool. 

---

## Quick Start

To build and run the project:

```bash
# Build and start the Docker environment
docker compose up --build

# Run the Rust application with your Excel folder, batch size, and tag
cargo run --release -- \
  --path path/to/folder/full/of/similar/excel/files/ \
  --batch-size 50000 \
  --tag my-cluster-tag
```

### Query Example:

In this case, all the files I uploaded had the legal name of the company in the cell 'D4' and the vat code in the cell 'J4' of the Sheet1, so:

```SQL
SELECT 
  files.tag AS file_cluster_name,
  anyIf(excel_cells.value, row_index = 3 AND col_index = 3) AS legal_name, 
  anyIf(excel_cells.value, row_index = 3 AND col_index = 9) AS vat_code 
FROM excel_cells 
JOIN excel_sheets 
ON 
  excel_sheets.sheet_id = excel_cells.sheet_id 
JOIN files
ON
  files.file_id = excel_sheets.file_id
WHERE 
  excel_sheets.sheet_name = 'Sheet1' AND 
  files.tag = 'test-2025-09-22'
GROUP BY files.file_id, files.tag;
```
should work.

Many parts are not complete yet, but the CLI is usable.
