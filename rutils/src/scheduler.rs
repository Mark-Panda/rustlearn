use anyhow::Context;
use cron::Schedule;
use std::str::FromStr;
use tokio_cron_scheduler::{Job, JobScheduler};

/// Cron scheduler
pub struct Scheduler {
    pub scheduler: JobScheduler,
}

impl Scheduler {
    pub async fn new() -> anyhow::Result<Self> {
        let scheduler = JobScheduler::new()
            .await
            .context("error while initializing the scheduler")?;
        Ok(Self { scheduler })
    }

    pub async fn add_job<F, Fut>(&self, cron_expr: &str, job_fn: F) -> anyhow::Result<()>
    where
        F: Fn() -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        Schedule::from_str(cron_expr)
            .context(format!("invalid cron expression format {}", cron_expr))?;

        let job = Job::new_async(cron_expr, move |_uuid, _l| {
            let job_fn = job_fn.clone();
            Box::pin(async move {
                job_fn().await;
            })
        })
        .context(format!(
            "error while creating job with cron expression: {}",
            cron_expr
        ))?;

        self.scheduler
            .add(job)
            .await
            .context("error while adding job to scheduler")?;

        Ok(())
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        self.scheduler
            .start()
            .await
            .context("error while starting scheduler")?;
        Ok(())
    }
}
