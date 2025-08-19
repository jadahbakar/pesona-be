use std::sync::Arc;

use async_trait::async_trait;
use config::Config;
use validator::Validate;

use crate::{
    app::error::AppError,
    auth::domain::{
        entity::user::User,
        inout::prelude::{LoginInput, LoginOutput},
    },
};

#[warn(dead_code)]
// #[derive(Clone)]
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
    #[warn(dead_code)]
    async fn authenticate_user(&self, email: &str, _password: &str) -> Result<User, AppError> {
        let _ = email;
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
