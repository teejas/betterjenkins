use actix_web::{App, HttpServer};
use dotenv::dotenv;
use k8s_openapi::api::batch::v1::Job;
use kube::{Api, Client, core::params::PostParams};
use tokio;
use sqlx::Row;
use serde_yaml;

use crate::config::connect_to_db;

pub async fn run_executors() -> i64 {
  let client = Client::try_default().await.unwrap();
  let jobs: Api<Job> = Api::namespaced(client, "betterjenkins");

  // let p = pods.get("betterjenkins-server-6645d8477c-c88wg").await.unwrap();
  // println!("Got betterjenkins-server pod with containers: {:?}", p.spec.unwrap().containers);

  let mut db = connect_to_db().await.unwrap();
  let mut tasks_count: i64 = 0;
  loop {
    let query_response = sqlx::query("SELECT COUNT(*) FROM tasks;")
      .fetch_one(&mut db)
      .await
      .unwrap();
    tasks_count = query_response.get::<i64, &str>("count");
    if tasks_count > 0 {
      println!("Got tasks count: {:?}", tasks_count);
      let f = std::fs::File::open("k8s_manifests/executor/executor-job.yaml").unwrap();
      let d: Job = serde_yaml::from_reader(f).unwrap();
      let pp = PostParams {
        dry_run: false,
        field_manager: None,
      };
      jobs.create(&pp, &d).await.unwrap();
    } else {
      std::thread::sleep(std::time::Duration::from_secs(5));
    }
  }
  tasks_count
}