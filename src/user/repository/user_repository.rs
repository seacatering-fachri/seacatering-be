use crate::user::model::user::User;
use sqlx::PgPool;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }

    pub async fn create_user(pool: &PgPool, user: &User) -> Result<User, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            "INSERT INTO users (full_name, email, password_hash, is_admin)
             VALUES ($1, $2, $3, $4) RETURNING *",
            user.full_name,
            user.email,
            user.password_hash,
            user.is_admin
        )
        .fetch_one(pool)
        .await?;
        Ok(result)
    }
}
