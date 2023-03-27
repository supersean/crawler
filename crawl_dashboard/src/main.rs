use std::{
    env,
    sync::{Mutex, MutexGuard},
};

use actix_cors::Cors;
use actix_web::{
    http::header::{self, ContentType},
    middleware::Logger,
    web::{scope, Data},
    App, HttpResponse, HttpServer,
};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use errors::api_error::ApiError;
use serde::Serialize;

pub mod api;
pub mod db;
pub mod errors;
pub mod repository;

pub struct AppState {
    conn: Mutex<PgConnection>,
}
impl AppState {
    pub fn get_conn(&self) -> Result<MutexGuard<PgConnection>, ApiError> {
        Ok(self
            .conn
            .lock()
            .or_else(|e| return Err(ApiError::ServerError(e.to_string())))?)
    }
}

pub fn ok_response<T>(body: &T) -> Result<HttpResponse, ApiError>
where
    T: Serialize,
{
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialize_object(body)?))
}
fn serialize_object<T>(object: &T) -> Result<String, ApiError>
where
    T: Serialize,
{
    Ok(serde_json::to_string(object)?)
}
fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let conn = establish_connection();
    let app_state = Data::new(AppState {
        conn: Mutex::new(conn),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600);
        let logger = Logger::default();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(app_state.clone())
            .service(scope("/api").configure(api::configure_scope))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn main() {
// let terms = vec!["PGP".to_string(), "GLOBAL".to_string()];
// let drivers: Vec<Box<dyn CrawlDriver>> = vec![Box::new(crawl_drivers::GoogleDriver::new())];
// let crawl_request = CrawlRequest::new(terms, drivers);
// println!("crawling with terms: {:#?}", crawl_request.terms);
// let responses = crawl_request.crawl();
// println!("responses: {:#?}", responses);
//
// }
