use tokio_cron_scheduler::{JobScheduler, Job};

// #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[tokio::main]
async fn main() {
    let sched = JobScheduler::new().await.unwrap();
  
    sched.add(Job::new("1/10 * * * * *", |_uuid, _l| {
        println!("I run every 10 seconds");
    }).unwrap()).await.unwrap();

    sched.start().await.unwrap();
  
    // Wait a while so that the jobs actually run
    tokio::time::sleep(core::time::Duration::from_secs(100)).await;
}