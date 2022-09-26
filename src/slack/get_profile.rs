use super::model::{Error, Profile};
use crate::model::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProfileResponse {
    ok: bool,
    profile: Option<Profile>,
    error: Option<String>,
}

pub async fn get_profile(user: &str, config: &Config) -> Result<Profile, Error> {
    let client = reqwest::Client::new();
    let url = format!("https://slack.com/api/users.profile.get?user={}", user);
    let response = client
        .get(&url)
        .header(
            "Authorization",
            format!("Bearer {}", config.slack_bot_token),
        )
        .send()
        .await?;

    if !response.status().is_success() {
        error!("Slack api returned error: {}", response.status());
        return Err(Error::SlackApiError(
            response.status().as_u16(),
            response.text().await?,
        ));
    }

    let text = response.text().await?;

    debug!("{}", text);

    let response = serde_json::from_str::<ProfileResponse>(&text)?;
    if !response.ok {
        error!("Slack api returned error: {}", response.error.unwrap());
        return Err(Error::SlackApiError(
            500,
            "Slack api returned error".to_string(),
        ));
    }

    Ok(response.profile.unwrap())
}
