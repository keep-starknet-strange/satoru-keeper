use sqlx::postgres::PgPool;
use sqlx::Error;

pub struct HeadChain {
    pool: PgPool,
}

impl HeadChain {
    pub fn new(pool: PgPool) -> Self {
        HeadChain { pool }
    }

    pub async fn get_last_block_indexed(&self) -> Result<i64, Error> {
        let row: (Option<i64>,) =
            sqlx::query_as("SELECT MAX(block_number) FROM last_indexed_block")
                .fetch_one(&self.pool)
                .await?;
        Ok(row.0.unwrap_or(0))
    }

    pub async fn update_last_block_indexed(&self, block_number: i64) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO last_indexed_block (block_number) VALUES ($1) ON CONFLICT (id) DO UPDATE SET block_number = $1",
            block_number
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
