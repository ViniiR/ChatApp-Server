use actix_web::{post, web::{self}, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct UserData {
    name: String,
    date: String,
    age: u32,
}

#[post("/post")]
async fn test(user_data: web::Json<UserData>) -> impl Responder {
    let mut modified_data = user_data.into_inner();
    modified_data.name = "Carlos Vinícios".to_string();
    modified_data.age = 20;
    HttpResponse::Ok().json(modified_data)
}
