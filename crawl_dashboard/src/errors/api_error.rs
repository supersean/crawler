use actix_web::http::StatusCode;
use actix_web::HttpResponse;

#[derive(Debug)]
pub enum ApiError {
    ServerError(String),
    NotFound(String),
    BadRequest(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log = match self {
            ApiError::ServerError(message) => format!("[SERVER ERROR] {}", message),
            ApiError::NotFound(message) => format!("[NOT FOUND] {}", message),
            ApiError::BadRequest(message) => format!("[BAD REQUEST] {}", message),
        };
        write!(f, "[ERROR] | {}", log)
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ApiError::ServerError(message) => HttpResponse::InternalServerError().json(message),
            ApiError::NotFound(message) => HttpResponse::NotFound().json(message),
            ApiError::BadRequest(message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(err: diesel::result::Error) -> ApiError {
        match err {
            _ => ApiError::ServerError(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        ApiError::ServerError(value.to_string())
    }
}
