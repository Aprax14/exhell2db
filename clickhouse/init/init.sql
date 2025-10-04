CREATE DATABASE IF NOT EXISTS chfiles;
USE chfiles;

CREATE TABLE IF NOT EXISTS files (
    file_id UUID DEFAULT generateUUIDv4(),
    tag String,
    file_name String,
    file_path String,
    uploaded_at_utc DateTime64(3, 'UTC') DEFAULT now64()
) ENGINE = MergeTree()
PRIMARY KEY (file_id)
ORDER BY (file_id, uploaded_at_utc);

CREATE TABLE IF NOT EXISTS excel_sheets (
    sheet_id UUID DEFAULT generateUUIDv4(),
    file_id UUID,
    sheet_name String
) ENGINE = MergeTree()
PRIMARY KEY (sheet_id)
ORDER BY (sheet_id, file_id);

CREATE TABLE IF NOT EXISTS excel_cells (
    cell_id UUID DEFAULT generateUUIDv4(),
    sheet_id UUID,
    row_index UInt32,
    col_index UInt32,
    value String,
    cell_type Enum8('String' = 1, 'Number' = 2, 'Boolean' = 3, 'Empty' = 4, 'DateTime' = 5, 'Error' = 6)
) ENGINE = MergeTree()
PRIMARY KEY (cell_id)
ORDER BY (cell_id, row_index, col_index);
