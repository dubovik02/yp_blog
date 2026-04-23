use sqlx::{PgPool, migrate, postgres::PgPoolOptions};

use crate::domain::error::ServerError;

pub async fn create_pool(database_url: &str) -> Result<PgPool, ServerError> {
    
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn create_schema(pool: &PgPool) -> Result<(), ServerError> {
    migrate!("./migrations").run(pool).await?;
    Ok(())
}