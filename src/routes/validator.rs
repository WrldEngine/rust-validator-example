use actix_web::{post, web, Responder, HttpResponse};
use sqlx::PgPool;

use crate::db::repository::{Repository, ValidationRepository};
use crate::schemas::validation_scheme;

use log::*;

#[post("/hash_exists/{hash}/")]
pub async fn hash_exists(hash: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {

    if pool.is_closed() {
        error!("ERROR: database connection is closed!");
        return HttpResponse::InternalServerError().finish()
    }

    let repository = ValidationRepository::new(&pool);
    match repository.add_or_exists(&hash).await {
        Ok(bool) => {
            let response = validation_scheme::StatusData {
                status: "success".to_string(),
                valid: !bool,
            };
            HttpResponse::Ok().json(response)
        }

        Err(_) => {
            error!("ERROR: Something went wrong with database");
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hash_exists);
}