use anyhow::Result;
use rutils::Scheduler;

pub struct CronJobs {
    scheduler: Scheduler,
}

impl CronJobs {
    pub async fn new() -> Result<Self> {
        let scheduler = Scheduler::new().await?;
        Ok(Self { scheduler })
    }

    pub async fn setup(&self) -> Result<()> {
        // 添加定时任务（每10秒执行一次）
        self.scheduler
            .add_job("1/10 * * * * *", || async {
                println!("执行定时任务10");
            })
            .await?;

        // 添加定时任务（每20秒执行一次）
        self.scheduler
            .add_job("1/20 * * * * *", || async {
                println!("执行定时任务20");
            })
            .await?;

        // 启动调度器
        self.scheduler.start().await?;

        Ok(())
    }
}
