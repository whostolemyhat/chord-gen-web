use actix_cors::Cors;
use actix_web::{
    error, get, http, middleware::Logger, post, web::Form, App, HttpResponse, HttpServer,
    Responder, Result,
};
use chord_gen::Chord;
use log::info;
use serde::Deserialize;
use std::env;

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
    let output_dir = env::var("OUTPUT_PATH").unwrap_or("./output".to_string());
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
    // let output_dir = "./output";
    match chord_gen::render(settings, &output_dir) {
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
    let port = env::var("PORT").unwrap_or("4041".to_string());

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Startng server");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1234")
            .allowed_origin("https://chordgenerator.xyz")
            .allowed_methods(vec!["GET", "POST", "HEAD"])
            //   .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(ping)
            .service(home)
            .service(handle_form)
    })
    .bind(format!("127.0.0.1:{:}", port))?
    .run()
    .await
}
