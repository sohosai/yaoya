use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "text")]
    Text {
        text: String,
        token: String,
        channel: String,
    },
    #[serde(rename = "block")]
    Blocks {
        blocks: Vec<Block>,
        token: String,
        channel: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Block {
    #[serde(rename = "section")]
    Section {
        text: Text,
    },
    Actions {
        elements: Vec<ActionElement>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ActionElement {
    #[serde(rename = "button")]
    Button {
        action_id: String,
        text: Text,
        value: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Text {
    #[serde(rename = "plain_text")]
    PlainText { text: String },
    #[serde(rename = "mrkdwn")]
    Markdown { text: String },
}
