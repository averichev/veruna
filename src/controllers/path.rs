use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::error::{InternalError};
use actix_web::http::StatusCode;
use repository::node_repository::{find_path};
use crate::AppState;
use crate::controllers::main_page_controller::main_page_action;
use crate::services::node_site_relation_service::get_node_site_relation;
use crate::services::site_service::get_site;

pub async fn path_test(
    req: HttpRequest,
    data: web::Data<AppState>,
)
    -> actix_web::Result<HttpResponse, Error>
{
    let conn = &data.conn;
    let path = req.path().to_string();
    let host = get_host(&req);
    let nodes = get_nodes(path.clone());

    let site_result = get_site(host, conn).await;
    if let Err(e) = site_result {
        return Ok(HttpResponse::from_error(e));
    }
    let site = site_result.unwrap();
    let site_id = site.id;

    let node_site_relation_result = get_node_site_relation(
        site.clone(),
        conn,
    ).await;

    if let Err(e) = node_site_relation_result {
        return Ok(HttpResponse::from_error(e));
    }

    let node_site_relation = node_site_relation_result.unwrap();

    let node_site_id = node_site_relation.site_id;
    let main_page_id = node_site_relation.node_id;

    if node_site_id != site_id {
        return Ok(HttpResponse::from_error(InternalError::new(
            format!("not found site for root node {}", path),
            StatusCode::NOT_FOUND,
        )));
    }

    let nodes_len = nodes.len();

    if nodes_len > 0 {
        let nodes_path_result = find_path(
            nodes,
            conn,
            main_page_id,
        ).await.unwrap();

        if nodes_path_result.len() != nodes_len {
            return Ok(HttpResponse::from_error(InternalError::new(
                format!("node not found {}", path),
                StatusCode::NOT_FOUND,
            )));
        }
        return Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(path.to_string()));
    }

    return main_page_action(conn, site, node_site_relation.node_id)
        .await;
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

fn get_host(request: &HttpRequest) -> String {
    let host_info: Vec<String> = request
        .connection_info()
        .host()
        .split(":")
        .map(|s| s.to_string())
        .filter(|v| !v.is_empty())
        .collect();
    let host = host_info.get(0).unwrap().to_string();
    host
}

fn get_nodes(path: String) -> Vec<String> {
    let nodes: Vec<String> = path
        .split("/")
        .map(|s| s.to_string())
        .filter(|v| !v.is_empty())
        .collect();
    nodes
}