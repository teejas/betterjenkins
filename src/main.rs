use actix_web::{App, HttpServer};

mod config;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(server::index)
            .service(server::process_upload)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}