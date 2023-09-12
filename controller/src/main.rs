use dotenv::dotenv;
use tokio;

mod config;
mod db;
mod server;
mod run;

mod threads;
use threads::start_threads;

#[tokio::main]
async fn main() {
  dotenv().ok();
  start_threads().await;
}