use minio::s3::args::{BucketExistsArgs, MakeBucketArgs, UploadObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;

use crate::db::{connect_to_db, DBConn};

pub struct WorkspaceManager<'a> {
  minio_client: Client<'a>,
  db: DBConn
}

impl WorkspaceManager<'_> {
  pub async fn new() -> Option<WorkspaceManager<'static>> {
    if let Some(mut db) = connect_to_db().await {
      // should connect to minio and pg db, create 
      let mut base_url = BaseUrl::from_string("betterjenkins-minio-server".to_string()).unwrap();
      base_url.https = true;

      let static_provider = StaticProvider::new(
        "minioadmin",
        "minioadmin",
        None,
      );

      let mut minio_client = Client::new(base_url.clone(), Some(&static_provider), None, None).unwrap();

      let bucket_name = "betterjenkins";

      let exists = minio_client
        .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
        .await
        .unwrap();

      if !exists {
        minio_client
          .make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap())
          .await
          .unwrap();
      }
      Some(WorkspaceManager {
        minio_client,
        db
      })
    } else {
      None
    }
  }

  pub async fn create_workspace_dirs(&self) {
    // should check jobs table and make sure there is a directory in the bucket for each job
    //  dir_name = name + "_" + job_count
    todo!()
  }

  pub async fn cleanup_workspace_dirs(&self) {
    // should check existing dirs and if there isnt a corresponding job in the jobs table, 
    //  delete the dir
    todo!()
  }
}