use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;

pub mod welcome;

#[derive(Deserialize)]
struct NameInput {
    name: String,
}

// // Handler for greeting
// async fn greet(name: web::Json<NameInput>) -> impl Responder {
//     HttpResponse::Ok().json(format!("{} hi", name.name))
// }

// Additional handler for saying goodbye
async fn goodbye(name: web::Json<NameInput>) -> impl Responder {
    HttpResponse::Ok().json(format!("Goodbye, {}", name.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    welcome::welcome();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .route("/backend", web::post().to(welcome::greet))    // Route for greeting
            .route("/goodbye", web::post().to(goodbye))  // Additional route for goodbye
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

   
}


