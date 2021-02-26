use std::fmt;

use crate::internal::*;

/// Postgres
pub struct Postgres {
    pool: deadpool_postgres::Pool,
}

impl Postgres {
    pub fn from_config(config: &Config) -> Result<Self> {
        let pool = config.postgres.create_pool(tokio_postgres::NoTls)?;

        // TODO: Check schema version here, how to work against external tools/changes?

        Ok(Self { pool })
    }

    /// Returns an error if queries can not be served
    pub async fn readiness(&self) -> Result<()> {
        let conn = self.pool.get().await?;
        let st = conn.prepare("SELECT 1 + 1").await?;
        conn.query_one(&st, &[]).await?;
        Ok(())
    }
}

impl fmt::Debug for Postgres {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Postgres").finish()
    }
}
