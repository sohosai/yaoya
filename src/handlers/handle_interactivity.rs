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
            return Ok(warp::reply::with_status(
                warp::reply::reply(),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    };

    let props = match serde_json::from_str::<model::Interactivity>(&props.payload) {
        Ok(param) => {
            info!("Interactivity payload JSON parsed");
            param
        }
        Err(e) => {
            println!("JSON parse error {}", e);
            return Ok(warp::reply::with_status(
                warp::reply::reply(),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    };

    match props {
        model::Interactivity::BlockActions {
            actions,
            response_url,
            ..
        } => {
            let mut response_message = "".to_string();

            if actions.len() != 1 {
                return Ok(warp::reply::with_status(
                    warp::reply(),
                    respond(
                        "不正な操作を検知しました。. actions.len != 1.",
                        &response_url,
                    )
                    .await,
                ));
            }

            match &actions[0] {
                ActionElement::Button {
                    action_id,
                    text,
                    value,
                    style,
                } => {
                    if action_id.contains("continue") {
                        match register(&config, &value).await {
                            Ok(_) => {
                                response_message =
                                    "ユーザが発行されました。メールを確認してください。"
                                        .to_string();
                            }
                            Err(e) => {
                                response_message = e;
                            }
                        }
                    } else if action_id.contains("cancel") {
                        response_message =
                            "操作をキャンセルしました。訂正してから再度操作を行ってください。"
                                .to_string();
                    } else {
                        response_message =
                            "不正な操作を検知しました。 Unknown action_id.".to_string()
                    }
                }
            }

            return Ok(warp::reply::with_status(
                warp::reply(),
                respond(&response_message, &response_url).await,
            ));
        }
        _ => Ok(warp::reply::with_status(
            warp::reply::reply(),
            warp::http::StatusCode::BAD_REQUEST,
        )),
    }
}

async fn register(config: &Config, prop_str: &str) -> Result<(), String> {
    let props = match serde_json::from_str::<InteractiveComponentValue>(prop_str) {
        Ok(param) => {
            info!("Interactivity payload JSON parsed");
            param
        }
        Err(e) => {
            println!("JSON parse error {}", e);
            return Err(
                "不正な操作を検出しました。 Interactiviy payload JSON parse error.".to_string(),
            );
        }
    };

    match props {
        InteractiveComponentValue::IsRealnameCorrectPromptAnswer {
            token,
            email,
            iat,
            user_id,
            real_name,
        } => {
            let params = negicloud::RegisterUserParams {
                userid: real_name.to_string(),
                password: "".to_string(),
                email: email,
                groups: vec!["実委人".to_string()],
            };

            match negicloud::register_user(&config, params).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    error!("{}", e);
                    return Err(
                        "エラー。リクエストは有効ですが、negicloudとの通信に失敗しました。"
                            .to_string(),
                    );
                }
            }
        }
    }
}

pub async fn respond(text: &str, url: &str) -> warp::http::StatusCode {
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(format!("{{\"text\":\"{}\"}}", text))
        .send()
        .await;

    match res {
        Ok(res) => {
            info!("Response sent");
            if res.status().is_success() {
                warp::http::StatusCode::OK
            } else {
                info!("Response failed");
                warp::http::StatusCode::BAD_REQUEST
            }
        }
        Err(e) => {
            info!("Response send error: {}", e);
            warp::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
