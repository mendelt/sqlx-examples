use sqlx::postgres::PgConnection;
use sqlx::{Connection, Executor};

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = PgConnection::connect(
        "postgresql://example-user:password123@localhost:5432/example-database",
    )
    .await?;

    conn.execute("BEGIN").await?;

    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&conn).await?;

    Ok(())
}
