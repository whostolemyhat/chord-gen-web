use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    error, get, http, middleware::Logger, post, web::Form, App, HttpResponse, HttpServer,
    Responder, Result,
};
use chord_gen::{get_filename, render_svg, Chord, Hand};
use log::info;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize, Debug)]
struct ChordForm {
    title: String,
    fingers: String,
    frets: String,
    hand: String,
}

#[derive(Serialize)]
struct ImageCreated {
    path: String,
}

#[post("/")]
async fn handle_form(payload: Form<ChordForm>) -> Result<impl Responder> {
    let output_dir = env::var("OUTPUT_PATH").unwrap_or_else(|_| "/static/output".to_string());
    info!("{:?}", payload);
    let frets: Vec<i32> = payload
        .frets
        .split(',')
        .map(|letter| letter.parse::<i32>().unwrap_or(-1))
        .collect();

    let fingers: Vec<&str> = payload.fingers.split(',').collect();
    let hand = match payload.hand.as_str() {
        "left" => Hand::Left,
        _ => Hand::Right,
    };

    let settings = Chord {
        frets,
        fingers,
        title: &payload.title,
        hand,
    };

    let filename = get_filename(&settings);
    if Path::new(&format!(".{}/{}.svg", output_dir, filename)).exists() {
        return Ok(HttpResponse::Ok().json(ImageCreated {
            path: filename.to_string(),
        }));
    }

    // only render if image doesn't exist yet
    match render_svg(settings, &format!(".{}", output_dir)) {
        Ok(_) => {
            let response = ImageCreated {
                path: filename.to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
}

// TODO logging/tracing
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "4041".to_string());

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
            .service(Files::new("/images", "./static/output"))
    })
    .bind(format!("127.0.0.1:{:}", port))?
    .run()
    .await
}
