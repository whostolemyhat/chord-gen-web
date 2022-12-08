use log::info;
// use clap::{arg, Command};
use actix_web::{
    dev::ResourcePath, error, get, middleware::Logger, post, web, App, HttpResponse, HttpServer,
    Responder, Result,
};
use chord_gen::Chord;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
async fn home() -> impl Responder {
    let settings = Chord {
        frets: vec![5, 7, 7, 6, 5, 5],
        fingers: vec!["1", "3", "4", "2", "1", "1"],
        size: 1,
        title: "A barre",
    };
    let output_dir = "./output";
    chord_gen::render(settings, output_dir);
    HttpResponse::Ok().body("Ok!")
}

// TODO logging/tracing
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Startng server");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(ping)
            .service(home)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
