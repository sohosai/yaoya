mod model;
use crate::{model::Config, slack::model::message::ActionElement};
use std::convert::Infallible;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct InteractivityPayload{
    payload: String
}

pub async fn handle_interactivity(
    body: String,
    _config: Config,
) -> Result<impl warp::Reply, Infallible> {
    info!("Handling interactivity");
    info!("{}",body);

    let props = match serde_urlencoded::from_str::<InteractivityPayload>(&body) {
        Ok(param) => {
            info!("Interactivity payload parsed");
            info!("{}",param.payload);
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
            println!("JSON parse error {}",e);
            return Ok(warp::reply::with_status(
                warp::reply::reply(),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    };

   

    match props {
        model::Interactivity::BlockActions {
            actions, .. 
        } => {

            
            for action in actions {
                match action {
                    ActionElement::Button {
                        action_id,
                        text,
                        value,
                        style,
                    } => {
                        register()
                    }
                }
            }

            Ok(warp::reply::with_status(
                warp::reply::reply(),
                warp::http::StatusCode::OK,
            ))
        }
        _ => Ok(warp::reply::with_status(
            warp::reply::reply(),
            warp::http::StatusCode::BAD_REQUEST,
        )),
    }
}
