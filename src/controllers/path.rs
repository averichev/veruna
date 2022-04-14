use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::error::{InternalError};
use actix_web::http::StatusCode;
use repository::host_repository::find_by_name;
use repository::host_site_repository::find_by_host_id;
use repository::node_repository::{find_node_by_code, find_node_site_relation, find_path};
use repository::site_repository::find_site_by_id;
use crate::AppState;
use crate::controllers::main_page_controller::main_page_action;

pub async fn path_test(
    req: HttpRequest,
    data: web::Data<AppState>
)
    -> actix_web::Result<HttpResponse, Error>
{
    let conn = &data.conn;
    let path = req
        .path()
        .to_string();
    let host_info: Vec<String> = req
        .connection_info()
        .host()
        .split(":")
        .map(|s| s.to_string())
        .filter(|v| !v.is_empty())
        .collect();
    let host = host_info.get(0).unwrap().to_string();
    let host_model_result = find_by_name(&host, conn)
        .await;

    if let Err(e) = host_model_result {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }
    let host_model_option = host_model_result.unwrap();
    if host_model_option.is_none() {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("host {} not found", &host.to_string()),
            StatusCode::NOT_FOUND,
        )));
    }

    let host_model = host_model_option.unwrap();
    let host_id = host_model.id;
    let host_site_result = find_by_host_id(
        host_id,
        conn
    )
        .await;

    if let Err(e) = host_site_result {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }

    let host_site_option = host_site_result.unwrap();

    if host_site_option.is_none() {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("host site relation not found by host id {}", host_id.to_string()),
            StatusCode::NOT_FOUND,
        )));
    }

    let host_site_model = host_site_option.unwrap();

    let site_result = find_site_by_id(
        host_site_model.site_id,
        conn
    )
        .await;

    if let Err(e) = site_result {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }

    let site_option = site_result.unwrap();



    if site_option.is_none() {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("site not found by id {}", host_site_model.site_id),
            StatusCode::NOT_FOUND,
        )));
    }

    let site = site_option.unwrap();

    let nodes: Vec<String> = path
        .split("/")
        .map(|s| s.to_string())
        .filter(|v| !v.is_empty())
        .collect();
    let len = nodes.len();
    if len == 0 {
        if path.eq("/") {
            return main_page_action(conn, site).await;
        }
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("Path not found {}", path),
            StatusCode::NOT_FOUND,
        )));
    }

    let root_node = nodes.get(0).unwrap();
    let root_node_result = find_node_by_code(root_node, conn)
        .await;
    if let Err(e) = root_node_result {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }
    let root_node_option = root_node_result.unwrap();
    if root_node_option.is_none() {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("root node not found by path {}", path),
            StatusCode::NOT_FOUND,
        )));
    }

    let root_node_model = root_node_option.unwrap();

    let node_site_relation_result = find_node_site_relation(root_node_model, conn)
        .await;

    if let Err(e) = node_site_relation_result {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )));
    }

    let node_site_relation_option = node_site_relation_result.unwrap();

    if node_site_relation_option.is_none() {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("not found site relation for root node {}", path),
            StatusCode::NOT_FOUND,
        )));
    }

    let node_site_relation_model = node_site_relation_option.unwrap();

    if node_site_relation_model.site_id != site.id {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("not found site for root node {}", path),
            StatusCode::NOT_FOUND,
        )));
    }

    let nodes_len = nodes.len();
    let nodes_path_result = find_path(nodes, conn)
        .await
        .unwrap();

    if nodes_path_result.len() != nodes_len {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("node not found {}", path),
            StatusCode::NOT_FOUND,
        )));
    }

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(path.to_string()))
}


pub async fn redirect_favicon()
    -> actix_web::Result<HttpResponse, Error>
{
    let response = HttpResponse::MovedPermanently()
        .insert_header(("Location", "/static/favicon.ico"))
        .finish();
    Ok(
        response
    )
}