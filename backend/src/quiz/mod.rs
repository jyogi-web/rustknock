
use actix_web::{App, get, HttpResponse, HttpServer, middleware::Logger, Responder, web};

mod quiz_json_reader;

#[get("/mod")]
pub async fn quiz() -> impl Responder {
    HttpResponse::Ok().json(quiz_json_reader::read_file())
}
