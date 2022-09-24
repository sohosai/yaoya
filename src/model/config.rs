use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub slack_signing_secret: String,
    pub slack_bot_token: String
}
