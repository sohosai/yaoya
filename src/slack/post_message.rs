use super::model::{Error, Message};

use std::collections::HashMap;

pub async fn post_message(token: &str, channel: &str, message: Message) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = "https://slack.com/api/chat.postMessage";

    let message_type = match message {
        Message::Text { .. } => "text",
        Message::Blocks { .. } => "blocks",
    }
    .to_string();

    let message_str = serde_json::to_string(&message)?;
    let request = HashMap::from([
        ("channel".to_string(), channel.to_string()),
        (message_type, message_str),
    ]);

    let res = client
        .post(url)
        .header("Content-Type", "application/www-form-urlencoded")
        .header("Authorization", format!("Bearer {}", token))
        .form(&request)
        .send()
        .await?;

    info!("{:?}", request);
    if res.status().is_success() {
        Ok(())
    } else {
        Err(Error::SlackApi(res.status().as_u16(), res.text().await?))
    }
}
