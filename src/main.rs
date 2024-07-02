mod routes;
mod db;
mod schemas;

use actix_web::{web, App, HttpServer};
use db::{database::create_pool, models::create_tables};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().await;
    let _ = create_tables(&pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::validator::init_routes)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}