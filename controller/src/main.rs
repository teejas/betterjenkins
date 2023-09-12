use actix_web::{rt, App, HttpServer, dev::Server};
use dotenv::dotenv;
use tokio::{self, signal, task};
use tokio_util::sync::CancellationToken;
use std::error::Error;

mod config;
mod db;
mod server;
mod run;
use run::run_executors;
mod threads;
use threads::start_threads;

#[tokio::main]
async fn main() {
  dotenv().ok();
  start_threads().await;
}