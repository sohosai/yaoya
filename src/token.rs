use crate::model::Config;
use chrono::{TimeZone, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenVerificationError {
    #[error("Invalid token")]
    Invalid,
    #[error("Token is expired")]
    Expired,
}

pub fn verify_token(
    token: &str,
    iat: u64,
    email: &str,
    config: &Config,
) -> Result<(), TokenVerificationError> {
    let token_basestring = format!("{}-{}-{}", iat, email, config.verify_salt);
    let computed_token = hmac_sha256::Hash::hash(token_basestring.as_bytes());
    let computed_token = hex::encode(computed_token);

    if computed_token != token {
        return Err(TokenVerificationError::Invalid);
    }

    let expire_at = Utc.timestamp(iat as i64, 0) + chrono::Duration::minutes(5);

    if expire_at.timestamp() < Utc::now().timestamp() {
        return Err(TokenVerificationError::Expired);
    }

    Ok(())
}
