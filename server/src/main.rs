extern crate log;
mod handler;
mod model;

use dotenv::dotenv;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::env;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to our Restaurant API.
        Available routes:
        POST /table/{id}/items -> create order for table. Body example: {"item_ids": [1, 2, 3]}
        GET /table/{id}/items?remaining=true -> show all remaining items for a specified table number.
        GET /table/{id}/items?remaining=false -> show all items for a specified table number.
        GET /table/{id}/items/{id} -> show a specified item for a specified table number.
        PUT /table/{id}/items/{id} -> update status of a specified item for a specified table number.
        DELETE /table/{id}/items/{id} -> remove a specified item for a specified table number.
    "#
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect_timeout(Duration::from_secs(1000))
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(handler::create)
            .service(handler::list)
            .service(handler::get)
            .service(handler::update_status)
            .service(handler::delete)
            .service(handler::paid)
            .route("/", web::get().to(index))
    });

    server.bind(format!("{}:{}", "0.0.0.0", "8080"))?
        .run()
        .await
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use actix_web::http::{Method, StatusCode};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Row;
    use serde_json::json;

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::with_uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_items_create() {

        // build with only one connection
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:password@0.0.0.0/test")
            .await
            .expect("pool failed");

        let mut app =
            test::init_service(App::new().data(pool.clone()).service(handler::create)).await;
        let req =
            test::TestRequest::with_uri("/table/4/items")
                .method(Method::POST)
                .set_json(&json!({"item_ids": [1,2,3]}))
                .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let count: i64 =
            sqlx::query("SELECT COUNT(id) as count from orders where table_number=4")
                .fetch_one(&pool)
                .await
                .expect("SELECT COUNT failed")
                .try_get("count")
                .unwrap();

        assert_eq!(count, 3);

        sqlx::query("DELETE from orders where table_number=4")
            .execute(&pool)
            .await
            .expect("DELETE FAILED");
    }

    #[actix_rt::test]
    async fn test_items_get_not_found() {

        // build with only one connection
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:password@0.0.0.0/test")
            .await
            .expect("pool failed");

        let mut app =
            test::init_service(App::new().data(pool.clone()).service(handler::get)).await;
        let req = test::TestRequest::with_uri("/table/1/items/100").method(Method::GET).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_items_get_ok() {

        // build with only one connection
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:password@0.0.0.0/test")
            .await
            .expect("pool failed");

        sqlx::query("INSERT INTO orders (item_id, table_number, preparation_time)\
         VALUES (1, 1, 10)")
            .execute(&pool)
            .await
            .expect("INSERT test failed");

        let mut app =
            test::init_service(App::new().data(pool.clone()).service(handler::get)).await;
        let req = test::TestRequest::with_uri("/table/1/items/1").method(Method::GET).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        sqlx::query("DELETE from orders where table_number=1 and item_id=1")
            .execute(&pool)
            .await
            .expect("DELETE FAILED");
    }

    #[actix_rt::test]
    async fn test_items_update() {

        // build with only one connection
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:password@0.0.0.0/test")
            .await
            .expect("pool failed");

        sqlx::query("INSERT INTO orders (item_id, table_number, preparation_time)\
         VALUES (1, 2, 10), (2,2,10), (3, 2, 10)")
            .execute(&pool)
            .await
            .expect("INSERT test failed");

        let mut app =
            test::init_service(App::new().data(pool.clone()).service(handler::update_status)).await;

        let req =
            test::TestRequest::with_uri("/table/2/items/1").method(Method::PUT).to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let count: i64 =
            sqlx::query("SELECT COUNT(id) as count from orders where table_number=2 and status='pending'")
            .fetch_one(&pool)
            .await
            .expect("SELECT COUNT failed")
            .try_get("count")
            .unwrap();

        assert_eq!(count, 2);

        let count: i64 =
            sqlx::query("SELECT COUNT(id) as count from orders where table_number=2 and status='served'")
            .fetch_one(&pool)
            .await
            .expect("SELECT COUNT failed")
            .try_get("count")
            .unwrap();

        assert_eq!(count, 1);

        sqlx::query("DELETE from orders where table_number=2")
            .execute(&pool)
            .await
            .expect("DELETE FAILED");
    }

    #[actix_rt::test]
    async fn test_items_delete() {

        // build with only one connection
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:password@0.0.0.0/test")
            .await
            .expect("pool failed");

        sqlx::query("INSERT INTO orders (item_id, table_number, preparation_time)\
         VALUES (1, 6, 10)")
            .execute(&pool)
            .await
            .expect("INSERT test failed");

        let mut app =
            test::init_service(App::new().data(pool.clone()).service(handler::delete)).await;
        let req = test::TestRequest::with_uri("/table/6/items/1").method(Method::DELETE).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let count: i64 = sqlx::query("SELECT COUNT(id) as count from orders where table_number=6 and item_id=1")
            .fetch_one(&pool)
            .await
            .expect("SELECT COUNT failed")
            .try_get("count")
            .unwrap();

        assert_eq!(count, 0);
    }

    #[actix_rt::test]
    async fn test_table_paid() {

        // build with only one connection
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:password@0.0.0.0/test")
            .await
            .expect("pool failed");

        sqlx::query("INSERT INTO orders (item_id, table_number, preparation_time)\
         VALUES (1, 5, 10), (2,5,10), (3, 5, 10)")
            .execute(&pool)
            .await
            .expect("INSERT test failed");

        let mut app =
            test::init_service(App::new().data(pool.clone()).service(handler::paid)).await;

        let req =
            test::TestRequest::with_uri("/table/5/paid").method(Method::DELETE).to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let count: i64 =
            sqlx::query("SELECT COUNT(id) as count from orders where table_number=5")
            .fetch_one(&pool)
            .await
            .expect("SELECT COUNT failed")
            .try_get("count")
            .unwrap();

        assert_eq!(count, 0);
    }
}