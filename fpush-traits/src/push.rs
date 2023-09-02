use async_trait::async_trait;
use derive_more::{Display, From};

pub type PushResult<T> = std::result::Result<T, PushError>;

#[derive(Debug, From, Display)]
pub enum PushError {
    CertLoading,
    PushEndpointTmp,
    PushEndpointPersistent,
    TokenRateLimited,
    TokenBlocked,
    Unknown(u16),
}

#[async_trait]
pub trait PushTrait {
    /// returns false if the token should be blocked
    async fn send(&self, token: String, notification: Option<PushPayload>) -> PushResult<()>;
}

#[derive(Default)]
pub struct PushPayload {
    pub title: Option<String>,
    pub body: Option<String>,
}
