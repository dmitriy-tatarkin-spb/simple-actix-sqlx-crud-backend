use serde::Serialize;
use actix_web::HttpResponse;
use anyhow::Error;

#[derive(Serialize)]
struct ErrorResponse { message: String }

pub fn internal_server_error(error: Error) -> HttpResponse {
  HttpResponse::InternalServerError().json(
    ErrorResponse { message: error.to_string() }
  )
}
