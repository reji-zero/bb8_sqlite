use bb8_sqlite::RusqliteConnectionManager;
use rusqlite::named_params;
use tokio::task;

const DATABASE_URL: &str = "sqlite://database.db?=mode=rwc";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manager = RusqliteConnectionManager::new(DATABASE_URL);
    let pool = bb8::Pool::builder().build(manager).await?;
    let conn = pool.get().await?;

    // rusqlite::Connection is synchronous, so good practice is to use
    // block_in_place() to ensure that we don't starve the tokio runtime of
    // available non-blocking threads to do work on. (Of course, in this trivial
    // example, there's no actual need for this.)
    let value = task::block_in_place(move || -> anyhow::Result<i32> {
        conn.execute("CREATE TABLE t (a INTEGER)", [])?;
        conn.execute(
            "INSERT INTO t (a) VALUES (:a)",
            named_params! {
                ":a": 42,
            },
        )?;

        Ok(conn.query_row("SELECT a FROM t", [], |row| row.get(0))?)
    })?;

    println!("we stored this value: {}", value);
    Ok(())
}
