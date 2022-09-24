use crate::model::Config;
use super::model::{Profile,Error};

pub async fn get_profile(user: &str,config: &Config) -> Result<Profile, Error> {
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
