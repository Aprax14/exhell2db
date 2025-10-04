use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Env {
    #[expect(unused)]
    app_name: String,
    #[expect(unused)]
    rust_log: String,
    clickhouse_user: String,
    clickhouse_password: String,
    clickhouse_db: String,
}

pub struct AppState {
    pub clickhouse_client: clickhouse::Client,
}

impl AppState {
    pub fn new(env: &Env) -> Self {
        Self {
            clickhouse_client: clickhouse::Client::default()
                .with_url("http://localhost:8123")
                .with_user(&env.clickhouse_user)
                .with_password(&env.clickhouse_password)
                .with_database(&env.clickhouse_db),
        }
    }
}
