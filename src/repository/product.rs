use diesel::query_dsl::methods::FindDsl;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{OptionalExtension, RunQueryDsl, SqliteConnection};
use veruna::models::Product;
use crate::DbError;

pub fn product_by_id(
    connection: PooledConnection<ConnectionManager<SqliteConnection>>,
    product_id: i32,
)
    -> Result<Option<Product>, DbError> {
    use veruna::schema::products::dsl::*;
    let product = products
        .find(product_id)
        .first(&connection)
        .optional()?;
    Ok(product)
}