use actix_web::{web, App, HttpServer};
mod handlers;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(db::get_client_sync())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}