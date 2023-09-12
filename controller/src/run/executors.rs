use k8s_openapi::api::batch::v1::Job;
use kube::{Api, Client, api::ListParams, core::params::PostParams};
use sqlx::Row;
use actix_web::{App, HttpServer};
use tokio::{self, signal, task};
use tokio_util::sync::CancellationToken;

use crate::db::connect_to_db;

pub async fn run_executors(last_tasks_count: i64) -> i64 {
  let client = Client::try_default().await.unwrap();
  let jobs: Api<Job> = Api::namespaced(client, "betterjenkins");

  if let Some(mut db) = connect_to_db().await {
    let query_response = sqlx::query("SELECT COUNT(*) FROM tasks;")
      .fetch_one(&mut db.conn)
      .await
      .unwrap();
    let tasks_count = query_response.get::<i64, &str>("count");

    println!("Got last tasks count: {:?}", last_tasks_count);
    println!("Got tasks count: {:?}", tasks_count);

    if tasks_count > 0 && tasks_count != last_tasks_count {
      let f = std::fs::File::open("k8s_manifests/executor/executor-job.yaml").unwrap();
      let mut d: Job = serde_yaml::from_reader(f).unwrap();

      let lp = ListParams::default().labels("app=betterjenkins-executor");
      let existing_jobs = jobs.list(&lp).await.unwrap();

      println!("Got jobs: {:?}", existing_jobs.items.len());
      let pp = PostParams {
        dry_run: false,
        field_manager: None,
      };
      d.metadata.name = Some(format!(
        "{}-{}", d.metadata.name.ok_or("unnamed").unwrap(), existing_jobs.items.len() + 1)
      );
      let create_job = jobs.create(&pp, &d).await;

      if let Err(err) = create_job {
        eprintln!("Failed to create job, error: {:?}", err);
        std::thread::sleep(std::time::Duration::from_secs(5));
      }

      tasks_count

    } else {
      println!("No tasks or last task not yet started");
      std::thread::sleep(std::time::Duration::from_secs(5));
      0
    }
  } else {
    std::thread::sleep(std::time::Duration::from_secs(5));
    -1
  }
}