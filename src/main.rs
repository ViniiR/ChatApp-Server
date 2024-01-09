mod route;

use crate::route::test;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(test))
        .bind("localhost:8080")?
        .run()
        .await
}
