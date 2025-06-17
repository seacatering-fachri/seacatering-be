mod auth;
mod user;

use actix_web::{web, App, HttpServer};
use auth::controller::auth_controller::{register, login};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&db_url).await.expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(register)
            .service(login)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
