#[cfg(test)]
mod tests {
    use chrono::Utc;
    use exhell_utils::excel::file::ChExcelFile;
    use uuid::Uuid;

    use crate::Env;

    #[tokio::test]
    async fn fake_file_insert() {
        let _ = dotenvy::dotenv();
        let env = Env::new().unwrap();

        let ch_conn = clickhouse::Client::default()
            .with_url("http://localhost:8123")
            .with_user(&env.clickhouse_user)
            .with_password(&env.clickhouse_password)
            .with_database(&env.clickhouse_db)
            .with_validation(true);

        let id = Uuid::new_v4();
        let fake_file = ChExcelFile {
            file_id: id,
            tag: String::from("test-from-rust"),
            file_name: String::from("fake_file.xlsx"),
            file_path: String::from("path/to/fake_file.xlsx"),
            uploaded_at_utc: Utc::now(),
        };

        let mut insert = ch_conn.insert::<ChExcelFile>("files").unwrap();
        insert.write(&fake_file).await.unwrap();
        insert.end().await.unwrap();

        ch_conn
            .query("DELETE FROM files WHERE file_id = ?")
            .bind(id)
            .execute()
            .await
            .unwrap();
    }
}
