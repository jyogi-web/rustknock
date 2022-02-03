use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer};
use build_timestamp::build_time;

mod quiz;
mod sample_ws;

build_time!("%A %Y-%m-%d / %H:%M:%S");

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    println!("built on: {}", BUILD_TIME);
    BUILD_TIME
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
            .service(web::resource("/").to(index))
            .service(quiz::quiz)
            .service(sample_ws::ws_index)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
