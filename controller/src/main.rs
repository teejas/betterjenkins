use dotenv::dotenv;

mod config;
mod db;
mod run;
use run::start_threads;
mod workspace;

#[tokio::main]
async fn main() {
  dotenv().ok();
  start_threads().await;
}