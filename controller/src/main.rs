use dotenv::dotenv;

mod config;
mod db;
mod run;
use run::Run;
mod workspace;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let r = Run;
  r.run().await;
}