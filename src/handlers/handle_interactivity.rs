mod model;
use crate::{model::Config, slack::model::message::ActionElement};
use std::convert::Infallible;

use super::verify_email::InteractiveComponentValue;
use crate::negicloud;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct InteractivityPayload {
    payload: String,
}

pub async fn handle_interactivity(
    body: String,
    config: Config,
) -> Result<impl warp::Reply, Infallible> {
    info!("Handling interactivity");
    info!("{}", body);

    let props = match serde_urlencoded::from_str::<InteractivityPayload>(&body) {
        Ok(param) => {
            info!("Interactivity payload parsed");
            info!("{}", param.payload);
            param
        }
        Err(e) => {
            println!("{:?}", e);
            return Ok("エラーが発生しました。Payload Parse error");
        }
    };

    let props = match serde_json::from_str::<model::Interactivity>(&props.payload) {
        Ok(param) => {
            info!("Interactivity payload JSON parsed");
            param
        }
        Err(e) => {
            println!("JSON parse error {}", e);
            return Ok("エラーが発生しました。JSON Parse error");
        }
    };

    let (actions, response_url) = match props {
        model::Interactivity::BlockActions {
            actions,
            response_url,
            ..
        } => (actions, response_url),
    };

    if actions.len() != 1 {
        return Ok("不正な操作を検知しました。. actions.len != 1.");
    }

    let (action_id, value) = match &actions[0] {
        ActionElement::Button {
            action_id, value, ..
        } => (action_id.to_string(), value.to_string()),
    };

    if action_id.contains("continue") {
        let props = match serde_json::from_str::<InteractiveComponentValue>(&value) {
            Ok(param) => {
                info!("Interactivity payload JSON parsed");
                param
            }
            Err(e) => {
                println!("JSON parse error {}", e);
                return Ok("不正な操作を検出しました。 Interactiviy payload JSON parse error.");
            }
        };

        match props {
            InteractiveComponentValue::IsRealnameCorrectPromptAnswer {
                token: _,
                email,
                iat: _,
                user_id: _,
                real_name,
            } => {
                let params = negicloud::RegisterUserParams {
                    userid: real_name.to_string(),
                    password: "".to_string(),
                    email,
                    groups: vec!["実委人".to_string()],
                };

                tokio::spawn(async move {
                    let msg = match negicloud::register_user(&config, params).await {
                        Ok(_) => "ユーザが発行されました。メールを確認してください。",
                        Err(e) => {
                            error!("{}", e);
                            "エラー。リクエストは有効ですが、negicloudとの通信に失敗しました。"
                        }
                    };

                    respond(msg, response_url, config).await
                });

                Ok("")
            }
        }
    } else if action_id.contains("cancel") {
        Ok("操作を中止しました。訂正して、はじめからやり直してください。")
    } else {
        Ok("不正な操作を検知しました。 Unknown action_id.")
    }
}

async fn respond(msg: &str, response_url: String, config: Config) {
    let client = reqwest::Client::new();
    let res = client
        .post(response_url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", config.slack_bot_token),
        )
        .body(format!("{{\"text\":\"{}\"}}", msg))
        .send()
        .await;

    match res {
        Ok(_) => info!("Response sent"),
        Err(e) => error!("Response failed: {}", e),
    }
}
