use std::sync::Arc;

use async_trait::async_trait;
use config::Config;
use validator::Validate;

use crate::{
    app::Error::AppError,
    auth::domain::{
        entity::user::User,
        inout::prelude::{LoginInput, LoginOutput},
    },
};

#[derive(Clone)]
pub struct AuthnService {
    config: Arc<Config>,
}

#[async_trait]
pub trait AuthnUseCase: Send + Sync {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, AppError>;
}

impl AuthnService {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    async fn authenticate_user(&self, email: &str, password: &str) -> Result<User, AppError> {
        // let user = self.
        todo!()
    }
}

#[async_trait]
impl AuthnUseCase for AuthnService {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, AppError> {
        input.validate()?;

        // let user = self.au
        todo!()
    }
}
