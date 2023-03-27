use actix_web::web;
pub mod entries;
pub mod users;

pub fn configure_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").configure(users::configure_scope));
    cfg.service(web::scope("/entries").configure(entries::configure_scope));
}
