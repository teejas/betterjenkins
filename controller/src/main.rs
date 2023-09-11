use actix_web::{App, HttpServer};
use dotenv::dotenv;
use k8s_openapi::api::batch::v1::Job;
use kube::{Api, Client, core::params::PostParams};
use tokio;
use sqlx::Row;
use serde_yaml;

mod config;
use config::connect_to_db;
mod server;
mod run;
use run::run_executors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  tokio::spawn(async {
    run_executors().await
  });
  
  HttpServer::new(|| {
    App::new()
      .service(server::index)
      .service(server::process_upload)
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}