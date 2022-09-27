use serde::{Deserialize, Serialize};

use crate::slack::model::{message::ActionElement, Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Interactivity {
    #[serde(rename = "block_actions")]
    BlockActions {
        user: User,
        api_app_id: String,
        token: String,
        trigger_id: String,
        response_url: String,
        actions: Vec<ActionElement>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    name: String,
}
