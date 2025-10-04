use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use tokio::sync::mpsc;

use crate::inserter::FileInserterBatch;

mod ch_tests;
mod inserter;

#[derive(Debug, Clone, Deserialize)]
pub struct Env {
    clickhouse_user: String,
    clickhouse_password: String,
    clickhouse_db: String,
}

impl Env {
    pub fn new() -> Result<Self, anyhow::Error> {
        envy::from_env::<Env>().context("bad environment variables")
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to folder containing xlsx files
    #[arg(short, long)]
    path: String,
    /// associate a tag to the files which can be used later
    /// to recognize all the files with the same structure
    #[arg(short, long)]
    tag: String,
    /// rows batch size before insert into DB
    #[arg(short, long, default_value_t = 50000)]
    batch_size: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let args = Args::parse();
    tracing_subscriber::fmt::init();

    let env = Env::new()?;
    let ch_conn = clickhouse::Client::default()
        .with_url("http://localhost:8123")
        .with_user(&env.clickhouse_user)
        .with_password(&env.clickhouse_password)
        .with_database(&env.clickhouse_db)
        .with_validation(true);

    tracing::info!("files insertion starting");

    let (tx, mut rx) = mpsc::channel::<exhell_utils::FileContent>(100);
    let mut batch = FileInserterBatch::new(args.batch_size);
    let mut counter = 0;
    let files = fs::read_dir(&args.path).context("failed to read given dir")?;
    for file in files {
        counter += 1;
        let path = file?.path();
        let tag = args.tag.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            match exhell_utils::FileContent::from_path(&path, &tag) {
                Ok(content) => {
                    if let Err(e) = tx.send(content).await {
                        tracing::error!(
                            "failed to send FileContent for {}: {e}",
                            path.to_string_lossy()
                        );
                    }
                }
                Err(e) => {
                    tracing::error!("failed to parse {:?}: {e}", path);
                }
            }
        });
    }
    drop(tx);

    while let Some(file_content) = rx.recv().await {
        batch.push(file_content, &ch_conn).await.unwrap();
    }
    batch.flush(&ch_conn).await?;
    tracing::info!("insertion completed. {} files saved.", counter);

    Ok(())
}
