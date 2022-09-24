use crate::model::Config;
use super::model::Profile;

use thiserror::Error;
#[derive(Debug, Error)]
pub enum SlackError{
    #[error("Error while sending requewst {0}")] 
    RequestError(#[from] reqwest::Error),
    #[error("Error while parsing response {0}")]
    ParseError(#[from] serde_urlencoded::de::Error)
}

pub async fn get_profile(user: &str,config: &Config) -> Result<Profile, SlackError> {
    let client = reqwest::Client::new();
    let url = format!("https://slack.com/api/users.profile.get?user={}", user);
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.slack_bot_token))
        .send().await?;

    let response = response.text().await?;
    let profile = serde_urlencoded::from_str::<Profile>(&response)?;
    Ok(profile)
}
