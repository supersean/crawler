
pub fn configure_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/crawls").service(all));
}

#[get("/")]
pub async fn all() -> Result<impl Responder, Error> {
    ok_response(())
}
