use std::sync::Arc;

use async_trait::async_trait;
use config::Config;

use crate::{
    app::Error::AppError,
    auth::domain::inout::prelude::{LoginInput, LoginOutput},
};

#[derive(Clone)]
pub struct AuthnService {
    config: Arc<Config>,
}

pub trait AuthnUseCase: Send + Sync {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, AppError>;
}

impl AuthnService {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl AuthnUseCase for AuthnService {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, AppError> {
        Ok(())
    }
}
