use actix_web::{HttpResponse, Responder, web};
use crate::models::User;
use serde_json::json;

pub async fn create_user(user: web::Json<User>) -> impl Responder {
    let response = json!({
        "name": user.name,
        "email": user.email
    });
    HttpResponse::Ok().json(response)
}
