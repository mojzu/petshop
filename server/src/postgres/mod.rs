//! # Postgres
//!
//! <https://cheatsheetseries.owasp.org/cheatsheets/Database_Security_Cheat_Sheet.html>
//! <https://cheatsheetseries.owasp.org/cheatsheets/SQL_Injection_Prevention_Cheat_Sheet.html>
use crate::internal::*;
use petshop_proto::api::{Fortune, World};
use std::fmt;

/// Postgres Pool
pub struct PostgresPool {
    pool: deadpool_postgres::Pool,
    metrics: Arc<Metrics>,
}

/// Postgres Client
///
/// FIXME: Is there a way to refactor this so that pool/clients can be used
/// with any query? Can deref pool client but this bypasses statement cache
pub struct PostgresClient {
    client: tokio_postgres::Client,
}

const CLIENT_CHECK: &str = "SELECT 1 + 1";

impl PostgresPool {
    pub fn from_config(config: &Config, metrics: Arc<Metrics>) -> Result<Self, XErr> {
        let pool = config.postgres.create_pool(tokio_postgres::NoTls)?;

        // TODO: Check schema version here, how to work against external tools/changes?

        Ok(Self { pool, metrics })
    }

    /// Returns an error if queries can not be served
    #[tracing::instrument(skip(self))]
    pub async fn readiness(&self) -> Result<(), XErr> {
        let client_check = self.check().await;
        self.metrics.postgres_ready(client_check.is_ok());
        client_check?;
        Ok(())
    }

    /// Wraps returning a client from pool to set ready metric
    async fn check(&self) -> Result<(), XErr> {
        let client = self.pool.get().await?;
        let st = client.prepare(CLIENT_CHECK).await?;
        client.query_one(&st, &[]).await?;
        Ok(())
    }

    /// Returns fortunes for TFB
    pub async fn db_fortunes(&self) -> Result<Vec<Fortune>, XErr> {
        let client = self.pool.get().await?;
        let st = client
            .prepare(
                "
                    SELECT id, message
                    FROM Fortune
                ",
            )
            .await?;
        let rows = client.query(&st, &[]).await?;
        let rows: Vec<Fortune> = rows
            .into_iter()
            .map(|row| Fortune {
                id: row.get(0),
                message: row.get(1),
            })
            .collect();
        Ok(rows)
    }

    /// Returns random row in World table for TFB
    pub async fn db_world(&self) -> Result<World, XErr> {
        let id: i32 = Self::db_random_id();
        self.db_world_by_id(id).await
    }

    /// Returns array of random rows from World table for TFB
    pub async fn db_world_queries(&self, queries: i32) -> Result<Vec<World>, XErr> {
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

    pub async fn db_world_updates(&self, queries: i32) -> Result<Vec<World>, XErr> {
        let mut worlds = self.db_world_queries(queries).await?;
        let mut world_ids = vec![0; queries as usize];
        let mut random_numbers = vec![0; queries as usize];

        for i in 0..worlds.len() {
            world_ids[i] = worlds[i].id;
            random_numbers[i] = Self::db_random_id();
            worlds[i].random_number = random_numbers[i];
        }

        let mut client = self.pool.get().await?;
        let transaction = client.transaction().await?;
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

    async fn db_world_by_id(&self, id: i32) -> Result<World, XErr> {
        let client = self.pool.get().await?;
        let st = client
            .prepare(
                "
                    SELECT id, randomNumber
                    FROM World
                    WHERE id = $1
                ",
            )
            .await?;
        let row = client.query_one(&st, &[&id]).await?;
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

impl PostgresClient {
    pub async fn from_config(config: &Config) -> Result<Self, XErr> {
        let pg_config = config.postgres.get_pg_config()?;
        let (client, connection) = pg_config.connect(tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                warn!("connection error: {:#}", err);
            }
        });

        Ok(Self { client })
    }

    pub async fn check(&self) -> Result<(), XErr> {
        let st = self.client.prepare(CLIENT_CHECK).await?;
        self.client.query_one(&st, &[]).await?;
        Ok(())
    }
}

impl fmt::Debug for PostgresPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PostgresPool").finish()
    }
}

impl fmt::Debug for PostgresClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PostgresClient").finish()
    }
}
