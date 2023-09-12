use minio::s3::args::{BucketExistsArgs, MakeBucketArgs, UploadObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;

struct WorkspaceManager {
  minio_api: ,
  db: DBConn
}

impl WorkspaceManager {
  pub async fn new() {
    // should connect to minio and pg db
    todo!()
  }

  pub async fn create_workspace_dirs() {
    // should check tasks table and make sure there is a directory in the bucket for each task
    todo!()
  }
}