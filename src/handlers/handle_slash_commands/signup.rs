use crate::model::Config;
use crate::slack::model::CommandInput;
use serde::Serialize;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignupError {}

impl Into<warp::http::StatusCode> for SignupError {
    fn into(self) -> warp::http::StatusCode {
        warp::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn signup(input: &CommandInput, config: &Config) -> Result<impl Serialize, SignupError> {
    let mut response = String::new();
    response.push_str("You signed up for the following events");

    Ok(response)
}
