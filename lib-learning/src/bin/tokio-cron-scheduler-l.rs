use std::{str::FromStr, time::Duration};

use chrono::Utc;
use cron::Schedule;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    let mut sched = JobScheduler::new().await?;

    // Add async job
    sched
        .add(
            // 默认使用的 Timezone 是 UTC，所以在具体执行的时候会与系统时间存在差异
            Job::new_async("@daily", |_uuid, mut _l| {
                Box::pin(async move {
                    println!("I run async every 7 seconds");
                })
            })?,
        )
        .await?;

    sched
        .add(Job::new_async("1/4 * * * * *", |uuid, mut l| {
            Box::pin(async move {
                info!("I run async every 4 seconds id {:?}", uuid);
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => info!("Next time for 4s is {:?}", ts),
                    _ => warn!("Could not get next tick for 4s job"),
                }
            })
        })?)
        .await?;

    // Add code to be run during/after shutdown
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    // Start the scheduler
    sched.start().await?;

    // Wait while the jobs run
    tokio::time::sleep(Duration::from_secs(100)).await;

    Ok(())
}

#[test]
fn test_cron_expression() {
    //               sec  min   hour   day of month   month   day of week   year
    let expression = "@daily";
    let schedule = Schedule::from_str(expression).unwrap();
    println!("Upcoming fire times:");
    for datetime in schedule.upcoming(chrono::Local).take(10) {
        println!("-> {}", datetime);
    }
}
