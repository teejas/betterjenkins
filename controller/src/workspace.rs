use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::BucketConfiguration;
use std::{env, error::Error, str};
use sqlx::Row;

use crate::db::{connect_to_db, DBConn};

pub struct WorkspaceManager {
  bucket: Bucket,
  db: DBConn
}

impl WorkspaceManager {
  pub async fn new() -> Result<WorkspaceManager, Box<dyn Error + Send + Sync>> {
    if let Some(mut db) = connect_to_db().await {
      // should connect to minio and pg db, create
      let creds: Credentials = Credentials {
        access_key: Some(env::var("MINIO_ACCESS_KEY").unwrap().to_string()), 
        secret_key: Some(env::var("MINIO_SECRET_KEY").unwrap().to_string()), 
        security_token: None, 
        session_token: None, 
        expiration: None
      };
      let b_conf = Bucket::new(
        "betterjenkins",
        Region::Custom {
          region: "".to_owned(),
          endpoint: env::var("MINIO_API").unwrap().to_owned(),
        },
        creds.clone()
      ).unwrap();
      let bucket = Bucket::with_path_style(
        &b_conf
      );
    
      // 2) Create bucket if does not exist
      if let Ok((_, code)) = bucket.head_object("/").await {
        println!("Response code: {:?}", code);
      } else {
        let create_result = Bucket::create_with_path_style(
          bucket.name.as_str(),
          bucket.region.clone(),
          creds.clone(),
          BucketConfiguration::default(),
        )
        .await?;
    
        println!(
          "=== Bucket created\n{} - {} - {}",
          bucket.name, create_result.response_code, create_result.response_text
        );
      }
      Ok(WorkspaceManager {
        bucket,
        db
      })
    } else {
      Err("Error connecting to the database")?
    }
  }

  pub async fn create_workspace_dirs(&mut self) -> 
    Result<(), Box<dyn Error + Send + Sync>> {
    // should check jobs table and make sure there is a directory in the bucket for each job
    //  dir_name = name + "_" + job_count
    let query_response = sqlx::query("SELECT name || '_' || job_count AS name FROM jobs;")
      .fetch_all(&mut self.db.conn)
      .await?;
    let mut q_iter = query_response.iter();
    while let Some(row) = q_iter.next() {
      let job = row.get::<String, &str>("name");
      println!("job: {:?}", job);
      self.bucket.put_object(
        format!("/{}/", job), 
        "Start of betterjenkins executor logfile".as_bytes()
      ).await;
    }
    Ok(())
  }

  pub async fn cleanup_workspace_dirs(&mut self) -> 
    Result<(), Box<dyn Error + Send + Sync>> {
    // should check existing dirs and if there isnt a corresponding job in the jobs table, 
    //  delete the dir
    let results = self.bucket.list(String::new(), None).await?;
    let mut r_iter = results.iter();
    while let Some(result) = r_iter.next() {
      let mut c_iter = result.contents.iter();
      while let Some(content) = c_iter.next() {
        let mut dir_name = content.key.clone();
        dir_name.pop(); // remove trailing slash
        println!("dir name: {:?}", dir_name);
        let query_response = sqlx::query("SELECT COUNT(*) FROM jobs WHERE name = $1;")
          .bind(&dir_name)
          .fetch_one(&mut self.db.conn)
          .await?;
        let job_count = query_response.get::<i64, &str>("count");
        println!("job count: {:?}", job_count);
        if job_count == 0 {
          self.bucket.delete_object(content.key.clone()).await?;
        }
      }
    }
    Ok(())
  }
}