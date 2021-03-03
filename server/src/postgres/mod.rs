//! # Postgres
//!
use crate::internal::*;
use petshop_proto::api::v1::World;
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

    /// Returns random row in World table for TFB
    pub async fn db_world(&self) -> Result<World, XError> {
        let id: i32 = Self::db_random_id();
        self.db_world_by_id(id).await
    }

    /// Returns array of random rows from World table for TFB
    pub async fn db_world_queries(&self, queries: i32) -> Result<Vec<World>, XError> {
        use futures::stream::futures_unordered::FuturesUnordered;
        use futures::StreamExt;

        let worlds = (0..queries)
            .map(|_| self.db_world())
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await;
        let worlds: Result<Vec<_>, _> = worlds.into_iter().collect();
        Ok(worlds?)
    }

    pub async fn db_world_updates(&self, queries: i32) -> Result<Vec<World>, XError> {
        let mut worlds = self.db_world_queries(queries).await?;
        let mut world_ids = vec![0; queries as usize];
        let mut random_numbers = vec![0; queries as usize];

        for i in 0..worlds.len() {
            world_ids[i] = worlds[i].id;
            random_numbers[i] = Self::db_random_id();
            worlds[i].random_number = random_numbers[i];
        }

        let mut conn = self.pool.get().await?;
        let transaction = conn.transaction().await?;
        transaction
            .batch_execute("SELECT pg_advisory_xact_lock(42)")
            .await?;
        let st = transaction
            .prepare(
                "
                    UPDATE World as w SET
                        randomNumber = args.randomNumber
                    FROM (
                        SELECT unnest($1::int[]) id, unnest($2::int[]) randomNumber
                    ) AS args
                    WHERE w.id = args.id
                ",
            )
            .await?;
        transaction
            .execute(&st, &[&world_ids, &random_numbers])
            .await?;
        transaction.commit().await?;

        Ok(worlds)
    }

    async fn db_world_by_id(&self, id: i32) -> Result<World, XError> {
        let conn = self.pool.get().await?;
        let st = conn
            .prepare(
                "
                    SELECT id, randomNumber
                    FROM World
                    WHERE id = $1
                ",
            )
            .await?;
        let row = conn.query_one(&st, &[&id]).await?;
        Ok(World {
            id: row.get(0),
            random_number: row.get(1),
        })
    }

    fn db_random_id() -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let id: i32 = rng.gen_range(1..10001);
        id
    }
}

impl fmt::Debug for Postgres {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Postgres").finish()
    }
}
