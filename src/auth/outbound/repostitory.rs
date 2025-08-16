use crate::{app::Error::AppError, auth::domain::entity::user::User};

async trait AuthRepository: Send + Sync {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
}