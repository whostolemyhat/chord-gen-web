use log::info;
// use clap::{arg, Command};
use actix_web::{
    error, get, middleware::Logger, post, web::Form, App, HttpResponse, HttpServer, Responder,
    Result,
};
use chord_gen::Chord;
use serde::Deserialize;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize, Debug)]
struct ChordForm {
    title: String,
    fingers: String,
    frets: String,
}

#[post("/")]
async fn handle_form(payload: Form<ChordForm>) -> Result<impl Responder> {
    info!("{:?}", payload);
    let frets: Vec<i32> = payload
        .frets
        .split(',')
        .map(|letter| letter.parse::<i32>().unwrap_or(-1))
        .collect();

    let fingers: Vec<&str> = payload.fingers.split(',').collect();

    let settings = Chord {
        frets,
        fingers,
        title: &payload.title,
    };
    let output_dir = "./output";
    match chord_gen::render(settings, output_dir) {
        Ok(_) => Ok(HttpResponse::Ok().body("Ok!")),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hi!")
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
            .service(handle_form)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
