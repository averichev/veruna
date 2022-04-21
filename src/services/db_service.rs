use std::collections::HashMap;
use actix_web::error::InternalError;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};
use serde_json::{Value, Map, Number};
use serde_json::map::Values;
use view::models::model_list::ModelList;
use view::models::model_list_item::ModelListItem;
use crate::services::internal_db_error;

pub async fn get_table_info(connection: &DatabaseConnection, table_name: String)
                            -> Result<HashMap<String, String>, InternalError<String>> {
    let backend = connection.get_database_backend();
    let stmt = format!("PRAGMA table_info({})", table_name);
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


pub async fn get_table_data(connection: &DatabaseConnection, table_name: String)
                            -> Result<ModelList, InternalError<String>> {
    let backend = connection.get_database_backend();
    let stmt = format!("SELECT * FROM {}", table_name);
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
    let query_res = query_res.unwrap();
    let mut list: ModelList = ModelList{
        list: Vec::new()
    };
    for re in query_res {
        let id_result: Result<i32, DbErr> = re.try_get("", "id");
        if let Err(e) = id_result {
            return Err(internal_db_error(e));
        }
        let name_result: Result<String, DbErr> = re.try_get("", "name");
        if let Err(e) = name_result {
            return Err(internal_db_error(e));
        }
        let content_result: Result<String, DbErr> = re.try_get("", "content");
        if let Err(e) = content_result {
            return Err(internal_db_error(e));
        }
        let name = name_result.unwrap();
        let id = id_result.unwrap();
        let content = content_result.unwrap();
        list.list.push(ModelListItem{
            key: "id".to_string(),
            value: id.to_string()
        });
        list.list.push(ModelListItem{
            key: "name".to_string(),
            value: name.to_string(),
        });
        list.list.push(ModelListItem{
            key: "content".to_string(),
            value: content.to_string()
        });
    }
    Ok(list)
}