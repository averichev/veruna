use actix_web::{error, web, Error};
use rusqlite::Statement;
use serde::{Deserialize, Serialize};
use std::{thread::sleep, time::Duration};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type PageResult = Result<Vec<Page>, rusqlite::Error>;


#[allow(clippy::enum_variant_names)]
pub enum Queries {
    GetById
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    id: i32,
    pub header: String,
}

fn get_page_by_id(conn: Connection) -> PageResult {
    let stmt = conn.prepare(
        "SELECT t.*
      FROM Pages t
      WHERE Id = 1
      LIMIT 1",
    )?;
    get_model(stmt)
}

fn get_model(mut statement: Statement) -> PageResult {
    statement
        .query_map([], |row| {
            Ok(Page {
                id: row.get(0)?,
                header: row.get(1)?,
            })
        })
        .and_then(Iterator::collect)
}

pub async fn execute(pool: &Pool, query: Queries) -> Result<Vec<Page>, Error> {
    let pool = pool.clone();

    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        match query {
            Queries::GetById => get_page_by_id(conn),
        }
    })
        .await?
        .map_err(error::ErrorInternalServerError)
}