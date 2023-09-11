use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod config;
mod db;
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