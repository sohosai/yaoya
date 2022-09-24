use crate::model::Config;
use crate::slack::model::CommandInput;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::StatusCode;

mod signup;
use signup::signup;

use serde::Serialize;
#[derive(Serialize)]
struct SimpleResponse {
    text: String,
}

pub async fn handle_slash_commands(
    body: HashMap<String, String>,
    config: Config,
) -> Result<impl warp::Reply, Infallible> {
    let input: CommandInput = match body.try_into() {
        Ok(input) => input,
        Err(e) => {
            error!("Error parsing command input: {:?}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&SimpleResponse {
                    text: "Error parsing command input".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    let result = match input.command().as_str() {
        "signup" => signup(&input, &config).await,
        _ => {
            error!("Unknown command: {}", input.command());
            return Ok(warp::reply::with_status(
                warp::reply::json(&SimpleResponse {
                    text: "Unknown command".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            StatusCode::OK,
        )),
        Err(e) => {
            error!("Error handling command: {:?}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&SimpleResponse {
                    text: "Error handling command".to_string(),
                }),
                e.into(),
            ))
        }
    }
}
