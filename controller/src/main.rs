use dotenv::dotenv;

mod config;
mod db;
mod threads;
use threads::start_threads;

#[tokio::main]
async fn main() {
  dotenv().ok();
  start_threads().await;
}