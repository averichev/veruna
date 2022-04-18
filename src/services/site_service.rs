use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use sea_orm::DbConn;
use repository::host_repository::find_by_name;
use repository::host_site_repository::find_by_host_id;
use repository::site_repository::find_site_by_id;

pub async fn get_site(host: String, connection: &DbConn)
    -> Result<entity::site::Model, InternalError<String>> {
    let host_model_result = find_by_name(
        &host,
        connection,
    ).await;

    if let Err(e) = host_model_result {
        return Err(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let host_model_option = host_model_result.unwrap();
    if host_model_option.is_none() {
        return Err(InternalError::new(
            format!("host {} not found", &host.to_string()),
            StatusCode::NOT_FOUND,
        ));
    }

    let host_model = host_model_option.unwrap();
    let host_id = host_model.id;
    let host_site_result = find_by_host_id(
        host_id,
        connection,
    )
        .await;

    if let Err(e) = host_site_result {
        return Err(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let host_site_option = host_site_result.unwrap();

    if host_site_option.is_none() {
        return Err(InternalError::new(
            format!("host site relation not found by host id {}", host_id.to_string()),
            StatusCode::NOT_FOUND,
        ));
    }

    let host_site_model = host_site_option.unwrap();

    let site_result = find_site_by_id(
        host_site_model.site_id,
        connection,
    )
        .await;

    if let Err(e) = site_result {
        return Err(InternalError::new(
            format!("DB error {}", e.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let site_option = site_result.unwrap();


    if site_option.is_none() {
        return Err(InternalError::new(
            format!("site not found by id {}", host_site_model.site_id),
            StatusCode::NOT_FOUND,
        ));
    }

    let site = site_option.unwrap();
    Ok(site)
}