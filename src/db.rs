use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub mod moment;

pub async fn build_db(database_url: &String) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::new()
        .filename(&database_url)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .read_only(false);

    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
