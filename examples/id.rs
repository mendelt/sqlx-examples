use async_trait::async_trait;
use sqlx::{FromRow, query, query_as};
use sqlx::postgres::{PgPool, PgDone};

const DB_URI: &str = "postgresql://example-user:password123@localhost:5432/example-database";

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect_db(DB_URI).await;

    Ok(())
}

#[derive(Debug, FromRow)]
struct Stuff {
    id: i64,
    name: String,
    value: String,
}

#[async_trait]
trait ValueTableRepo {
    fn database(&self) -> &PgPool;

    async fn store(
        &self,
        name: String,
        value: String,
    ) -> Result<PgDone, sqlx::Error> {
        query("INSERT INTO example_table (name, value) VALUES (?, ?);")
            .bind(name)
            .bind(value)
            .execute(self.database())
            .await
    }

    async fn list(&self) -> Result<Vec<Stuff>, sqlx::Error> {
        query_as::<_, Stuff>("SELECT id, name, value FROM example_table")
            .fetch_all(self.database())
            .await
    }
}

impl ValueTableRepo for PgPool {
    fn database(&self) -> &PgPool {
        &self
    }
}

async fn connect_db(uri: &str) -> PgPool {
    PgPool::connect(uri).await.unwrap()
}
