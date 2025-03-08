pub fn welcome(){
    println!("hi");
}

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NameInput {
    pub name: String,
}

// Handler for greeting
pub async fn greet(name: web::Json<NameInput>) -> impl Responder {
    HttpResponse::Ok().json(format!("{} hi", name.name))
}
