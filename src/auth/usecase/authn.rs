use std::sync::Arc;

use config::Config;

use crate::auth::domain::inout::prelude::{LoginInput, LoginOutput};

#[derive(Clone)]
pub struct AuthnService {
    config: Arc<Config>,
}

pub trait AuthnUsecase: Send + Sync {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, AppError>
}