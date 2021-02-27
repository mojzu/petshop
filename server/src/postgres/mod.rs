//! # Postgres
//!
use crate::internal::*;
use std::fmt;

/// Postgres
pub struct Postgres {
    metrics: Arc<Metrics>,
    pool: deadpool_postgres::Pool,
}

impl Postgres {
    pub fn from_config(config: &Config, metrics: Arc<Metrics>) -> Result<Self, XError> {
        let pool = config.postgres.create_pool(tokio_postgres::NoTls)?;

        // TODO: Check schema version here, how to work against external tools/changes?

        Ok(Self { metrics, pool })
    }

    /// Returns an error if queries can not be served
    #[tracing::instrument(skip(self))]
    pub async fn readiness(&self) -> Result<(), XError> {
        let conn_query = self.conn_query_test().await;
        self.metrics.postgres_ready_set(conn_query.is_ok());
        conn_query?;
        Ok(())
    }

    /// Wraps returning a client from pool to set ready metric
    async fn conn_query_test(&self) -> Result<(), XError> {
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
