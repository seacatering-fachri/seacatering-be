use crate::user::model::user::User;
use crate::user::repository::user_repository::UserRepository;
use sqlx::PgPool;
use argon2::{self, Config};
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub async fn register(
        pool: &PgPool,
        full_name: String,
        email: String,
        password: String,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let salt = Uuid::new_v4().to_string();
        let config = Config::default();
        let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;

        let user = User {
            id: 0, // diabaikan karena auto-increment
            full_name,
            email: email.clone(),
            password_hash: hash,
            is_admin: false,
        };

        let created = UserRepository::create_user(pool, &user).await?;
        Ok(created)
    }

    pub async fn login(
        pool: &PgPool,
        email: String,
        password: String,
    ) -> Result<User, String> {
        if let Some(user) = UserRepository::find_by_email(pool, &email)
            .await
            .map_err(|e| e.to_string())?
        {
            let is_valid = argon2::verify_encoded(&user.password_hash, password.as_bytes())
                .map_err(|e| e.to_string())?;

            if is_valid {
                Ok(user)
            } else {
                Err("Invalid password".into())
            }
        } else {
            Err("User not found".into())
        }
    }
}
