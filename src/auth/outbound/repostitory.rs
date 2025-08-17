use crate::{app::Error::AppError, auth::domain::entity::user::User};

const GET_USER_BY_ID: &str = "SELECT user_status, user_company, user_name, user_password, user_email, user_telp FROM sec.user ";

trait AuthRepository: Send + Sync {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
}
