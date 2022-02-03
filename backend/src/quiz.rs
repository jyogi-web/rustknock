use actix_web::{get, web, HttpResponse, Responder};
use quiz_json::Quiz;
use rand::{
    prelude::{SliceRandom, StdRng},
    SeedableRng,
};

#[get("/quiz")]
pub async fn quiz() -> impl Responder {
    HttpResponse::Ok().json(quiz_json::read_file("quizzes.json"))
}

#[get("/quiz/{num}")]
pub async fn get_quiz_num(num: web::Path<usize>) -> impl Responder {
    let quizzes = quiz_json::read_file("quizzes.json");

    let mut rng = StdRng::from_rng(rand::thread_rng()).unwrap();
    let select_quizzes: Vec<Quiz> = quizzes
        .choose_multiple(&mut rng, num.to_owned())
        .cloned()
        .collect();

    HttpResponse::Ok().json(select_quizzes)
}
