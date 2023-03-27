use actix_web::{get, post, web, Responder};

use crate::{errors::api_error::ApiError, ok_response, repository::entry_repository, AppState};

pub fn configure_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(all).service(create);
}

#[get("/")]
pub async fn all(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let mut conn = data.get_conn()?;

    let entries = entry_repository::get_all_entries_for_user(&mut conn, 1)?;

    ok_response(&entries)
}

#[post("/")]
pub async fn create(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let mut conn = data.get_conn()?;

    ok_response(&"create entry".to_string())
}
