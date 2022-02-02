mod quiz_json_reader;

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[get("/quiz")]
async fn quiz() -> impl Responder {
    HttpResponse::Ok().json(quiz_json_reader::read_file())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("All")
                    .send_wildcard()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(quiz)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
