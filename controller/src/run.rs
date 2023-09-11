use k8s_openapi::api::batch::v1::Job;
use kube::{Api, Client, api::ListParams, core::params::PostParams};
use sqlx::Row;

use crate::db::DBConn;

pub async fn run_executors() -> i64 {
  let client = Client::try_default().await.unwrap();
  let jobs: Api<Job> = Api::namespaced(client, "betterjenkins");

  let mut db: DBConn = DBConn::new().await.unwrap();

  let mut tasks_count: i64 = 0;
  loop {
    let query_response = sqlx::query("SELECT COUNT(*) FROM tasks;")
      .fetch_one(&mut db.conn)
      .await
      .unwrap();
    let q_tasks_count = query_response.get::<i64, &str>("count");
    if q_tasks_count > 0 && tasks_count != q_tasks_count {
      tasks_count = q_tasks_count;
      println!("Got tasks count: {:?}", tasks_count);
      let f = std::fs::File::open("k8s_manifests/executor/executor-job.yaml").unwrap();
      let mut d: Job = serde_yaml::from_reader(f).unwrap();
      let lp = ListParams::default().labels("app=betterjenkins-executor");
      let existing_jobs = jobs.list(&lp).await.unwrap();
      println!("Got jobs: {:?}", existing_jobs.items.len());
      let pp = PostParams {
        dry_run: false,
        field_manager: None,
      };
      // let j = jobs.get("betterjenkins-server-6645d8477c-c88wg").await.unwrap();
      // println!("Got betterjenkins-server pod with containers: {:?}", p.spec.unwrap().containers);
      d.metadata.name = Some(format!("{}-{}", d.metadata.name.ok_or("unnamed").unwrap(), existing_jobs.items.len() + 1));
      let create_job = jobs.create(&pp, &d).await;
      if let Err(err) = create_job {
        eprintln!("Failed to create job, error: {:?}", err);
        std::thread::sleep(std::time::Duration::from_secs(5));
      }
    } else {
      println!("No tasks or last task not completed");
      std::thread::sleep(std::time::Duration::from_secs(5));
      tasks_count = 0;
    }
  }
}