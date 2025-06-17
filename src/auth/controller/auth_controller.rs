use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::Deserialize;

use crate::auth::service::auth_service::AuthService;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    match AuthService::register(
        &pool,
        req.full_name.clone(),
        req.email.clone(),
        req.password.clone(),
    )
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    match AuthService::login(&pool, req.email.clone(), req.password.clone()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}
