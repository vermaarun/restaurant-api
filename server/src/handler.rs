use actix_web::{web, HttpResponse, Responder, post, get, delete, put};
use sqlx::PgPool;
use crate::model::{OrderRequest, Info, Order, NotFound};

// Request Handlers

#[post("/table/{tid}/items")]
pub async fn create(
    path: web::Path<i32>,
    items: web::Json<OrderRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let item_list = items.into_inner().item_ids;
    let tid = path.into_inner();
    let mut orders = vec![];
    for item in item_list {
        let result = Order::create(tid, item, db_pool.get_ref()).await;
        match result {
            Ok(order) => orders.push(order),
            _ => {}
        }
    }
    HttpResponse::Created().json(orders)
}

#[get("/table/{tid}/items")]
pub async fn list(
    path: web::Path<i32>,
    info: web::Query<Info>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let table_id = path.into_inner();
    let remaining = info.remaining;

    if remaining == true {
        let result =
            Order::find_all_remaining(table_id, db_pool.get_ref()).await;
        match result {
            Ok(orders) => HttpResponse::Ok().json(orders),
            _ => HttpResponse::NotFound().json(NotFound { message: "Items not found.".to_string() }),
        }
    } else {
        let result = Order::find_all(
            table_id, db_pool.get_ref(),
        ).await;
        match result {
            Ok(orders) => HttpResponse::Ok().json(orders),
            _ => HttpResponse::NotFound().json(NotFound { message: "Items not found.".to_string() }),
        }
    }
}

#[get("/table/{tid}/items/{iid}")]
pub async fn get(
    path: web::Path<(i32, i32)>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let (table_id, item_id) = path.into_inner();
    let result = Order::find_by_id(
        table_id, item_id, db_pool.get_ref(),
    ).await;
    match result {
        Ok(order) => HttpResponse::Ok().json(order),
        _ => HttpResponse::NotFound().json(NotFound { message: "Item not found.".to_string() }),
    }
}

#[put("/table/{tid}/items/{iid}")]
pub async fn update_status(
    path: web::Path<(i32, i32)>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let (table_id, item_id) = path.into_inner();
    let result = Order::update(
        table_id, item_id, db_pool.get_ref(),
    ).await;
    match result {
        Ok(order) => HttpResponse::Ok().json(order),
        _ => HttpResponse::NotFound().json(NotFound { message: "Item not found.".to_string() }),
    }
}

#[delete("/table/{tid}/items/{iid}")]
pub async fn delete(
    path: web::Path<(i32, i32)>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let (table_id, item_id) = path.into_inner();
    let _ = Order::delete(table_id, item_id, db_pool.get_ref()).await;
    HttpResponse::Ok().json("Item Deleted")
}

// This path is for admin use; to make table ready for new customers
#[delete("/table/{tid}/paid")]
pub async fn paid(
    path: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let table_id = path.into_inner();
    let _ = Order::delete_all_rows(table_id, db_pool.get_ref()).await;
    HttpResponse::NoContent()
}
