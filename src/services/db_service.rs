use std::collections::HashMap;
use actix_web::error::InternalError;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};
use crate::services::internal_db_error;

pub async fn get_table_info(connection: &DatabaseConnection, table_name: String)
                            -> Result<HashMap<String, String>, InternalError<String>> {
    let backend = connection.get_database_backend();
    let stmt = format!("PRAGMA table_info({});", table_name);
    let statement = Statement::from_string(
        backend,
        stmt,
    );
    let query_res = connection
        .query_all(statement)
        .await;
    if let Err(e) = query_res {
        return Err(internal_db_error(e));
    }
    let mut table_info: HashMap<String, String> = HashMap::new();
    let query_res = query_res.unwrap();
    for re in query_res {
        let name_result: Result<String, DbErr> = re.try_get("", "name");
        if let Err(e) = name_result {
            return Err(internal_db_error(e));
        }
        let type_result: Result<String, DbErr> = re.try_get("", "type");
        if let Err(e) = type_result {
            return Err(internal_db_error(e));
        }
        let column_name = name_result.unwrap();
        let column_type = type_result.unwrap();
        table_info.insert(column_name, column_type);
    }
    Ok(table_info)
}