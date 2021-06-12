use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};
use anyhow::Result;
use sqlx::postgres::{PgQueryResult, PgRow};
use rand::Rng;

#[derive(Deserialize)]
pub struct Info {
    pub remaining: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OrderRequest {
    pub item_ids: Vec<i32>
}

#[derive(Serialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub item_id: i32,
    pub table_number: i32,
    pub preparation_time: i32,
    pub status: String,
}

#[derive(Serialize)]
pub struct NotFound {
    pub message: String,
}

#[derive(Serialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: i32,
}

// Implement create/get/update/delete from Order
impl Order {
    pub async fn create(tid: i32, iid: i32, pool: &PgPool) -> Result<Order> {
        let mut rng = rand::thread_rng();
        let time_taken = rng.gen_range(5..16);
        let mut tx = pool.begin().await?;
        let order =
            sqlx::query("INSERT INTO orders (item_id, table_number, preparation_time) \
                                    VALUES ($1, $2, $3) \
                                    RETURNING id, item_id, table_number, preparation_time, status")
                .bind(iid)
                .bind(tid)
                .bind(time_taken)
                .map(|row: PgRow| {
                    Order {
                        id: row.get(0),
                        item_id: row.get(1),
                        table_number: row.get(2),
                        preparation_time: row.get(3),
                        status: row.get(4),
                    }
                })
                .fetch_one(&mut tx)
                .await?;

        tx.commit().await?;
        Ok(order)
    }

    pub async fn find_by_id(tid: i32, iid: i32, pool: &PgPool) -> Result<Order> {
        let rec = sqlx::query!(
            r#"
                    SELECT * FROM orders WHERE table_number = $1 and item_id = $2
                "#,
            tid, iid
        )
            .fetch_one(&*pool)
            .await?;

        Ok(Order {
            id: rec.id,
            item_id: rec.item_id,
            table_number: rec.table_number,
            preparation_time: rec.preparation_time,
            status: rec.status,
        })
    }

    pub async fn find_all_remaining(tid: i32, pool: &PgPool) -> Result<Vec<Order>> {
        let mut orders = vec![];
        let recs = sqlx::query!(
            r#"
                    SELECT * FROM orders WHERE table_number = $1 and status=$2
                "#,
        tid, "pending"
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            orders.push(Order {
                id: rec.id,
                item_id: rec.item_id,
                table_number: rec.table_number,
                preparation_time: rec.preparation_time,
                status: rec.status,
            })
        }
        Ok(orders)
    }

    pub async fn find_all(tid: i32, pool: &PgPool) -> Result<Vec<Order>> {
        let mut orders = vec![];
        let recs = sqlx::query!(
            r#"
                    SELECT * FROM orders WHERE table_number = $1
                "#,
        tid
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            orders.push(Order {
                id: rec.id,
                item_id: rec.item_id,
                table_number: rec.table_number,
                preparation_time: rec.preparation_time,
                status: rec.status,
            })
        }

        Ok(orders)
    }

    pub async fn update(tid: i32, iid: i32, pool: &PgPool) -> Result<Order> {
        let mut tx = pool.begin().await.unwrap();
        let order =
            sqlx::query("UPDATE orders SET status = 'served' \
            WHERE id IN (SELECT id FROM orders WHERE table_number = $1 and item_id = $2 LIMIT 1) \
            RETURNING id, item_id, table_number, preparation_time, status")
                .bind(tid)
                .bind(iid)
                .map(|row: PgRow| {
                    Order {
                        id: row.get(0),
                        item_id: row.get(1),
                        table_number: row.get(2),
                        preparation_time: row.get(3),
                        status: row.get(4),
                    }
                })
                .fetch_one(&mut tx)
                .await?;

        tx.commit().await.unwrap();
        Ok(order)
    }

    pub async fn delete(tid: i32, iid: i32, pool: &PgPool) -> Result<PgQueryResult> {
        let mut tx = pool.begin().await?;

        let deleted =
            sqlx::query("DELETE FROM orders WHERE id IN (SELECT id FROM orders \
             WHERE table_number = $1 and item_id = $2 LIMIT 1)")
                .bind(tid)
                .bind(iid)
                .execute(&mut tx)
                .await?;

        tx.commit().await?;

        Ok(deleted)
    }

    pub async fn delete_all_rows(tid: i32, pool: &PgPool) -> Result<PgQueryResult> {
        let mut tx = pool.begin().await?;

        let deleted =
            sqlx::query("DELETE FROM orders where table_number = $1")
                .bind(tid)
                .execute(&mut tx)
                .await?;

        tx.commit().await?;

        Ok(deleted)
    }
}