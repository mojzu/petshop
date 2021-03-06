//! # Jobs
//!
//! An example using cron to run jobs can be found in examples/cron
//!
//! TODO: Example integration with k8s/nomad/systemd/etc?
use crate::internal::*;

/// Jobs
pub struct Jobs;

impl Jobs {
    /// Run job with name
    pub async fn run(config: Config, job: &str) -> Result<()> {
        match job {
            "example" => Self::example(&config).await,
            _ => Err(XError::jobs("job not found").into()),
        }
    }

    #[tracing::instrument(skip(config))]
    pub async fn example(config: &Config) -> Result<()> {
        info!("starting Jobs::example");

        let pg = PostgresClient::from_config(config).await?;
        pg.check().await?;

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        info!("finishing Jobs::example");
        Ok(())
    }
}
