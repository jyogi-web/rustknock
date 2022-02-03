use actix_web::{get, HttpResponse, Responder};

mod quiz_json_reader;

#[get("/quiz")]
pub async fn quiz() -> impl Responder {
    HttpResponse::Ok().json(quiz_json::read_file("quizzes.json"))
}
