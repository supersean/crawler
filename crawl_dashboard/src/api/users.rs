use actix_web::{get, post, web, Responder};

use crate::{
    db::models::NewUser, errors::api_error::ApiError, ok_response, repository::user_repository,
    AppState,
};

pub fn configure_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(all).service(create);
}

#[get("/")]
pub async fn all(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let mut conn = data.get_conn()?;

    let users = user_repository::get_all_users(&mut conn)?;

    ok_response(&users)
}

#[post("/")]
pub async fn create(
    data: web::Data<AppState>,
    new_user: web::Json<NewUser>,
) -> Result<impl Responder, ApiError> {
    let mut conn = data.get_conn()?;
    let new_user = user_repository::create_user(&mut conn, &new_user)?;
    ok_response(&new_user)
}
