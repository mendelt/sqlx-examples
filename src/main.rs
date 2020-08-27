use sqlx::postgres::PgConnection;
use sqlx::Connection;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let conn = PgConnection::connect(
        "postgresql://example-user:password123@localhost:5432/example-database",
    )
    .await;
    Ok(())
}
