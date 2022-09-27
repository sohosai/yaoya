use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub slack_signing_secret: String,
    pub slack_bot_token: String,
    pub sendgrid_token: String,
    pub my_baseurl: String,
    pub verify_salt: String,
    pub email_from: String,
    pub negicloud_base: String,
    pub negicloud_admin_user: String,
    pub negicloud_admin_password: String,
}
