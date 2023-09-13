use dotenv::dotenv;

mod config;
mod db;
mod run;
use run::Run;
mod workspace;

#[tokio::main]
async fn main() {
  dotenv().ok();
  if let Ok(mut r) = Run::new().await {
    r.run().await;
  }
}