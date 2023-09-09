use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod config;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    HttpServer::new(|| {
        App::new()
            .service(server::index)
            .service(server::process_upload)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}