use actix_web::{App, HttpServer};

mod dto;
mod api;
mod trade;

use api::get_max_and_min_prices;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor rodando em http://localhost:8080");
    HttpServer::new(|| App::new().service(get_max_and_min_prices))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
