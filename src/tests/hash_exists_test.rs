use crate::db::database::create_pool;
use crate::routes::validator::hash_exists;

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use super::*;
    use sqlx::{Pool, Postgres};

    #[actix_rt::test]
    async fn test_hash_exists() {
        let pool = create_pool().await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(hash_exists)
        ).await;

        let req = test::TestRequest::post()
            .uri("/hash_exists/test_hash")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let response_body: validation_scheme::StatusData = test::read_body_json(resp).await;
        assert_eq!(response_body.status, "success");
    }
}
