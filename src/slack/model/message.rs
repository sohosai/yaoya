use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Message {
    Text(String),
    #[serde(rename = "block")]
    Blocks(Vec<Block>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Block {
    #[serde(rename = "section")]
    Section { text: Text },
    #[serde(rename = "actions")]
    Actions { elements: Vec<ActionElement> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ActionElement {
    #[serde(rename = "button")]
    Button {
        action_id: String,
        text: Text,
        value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<ButtonStyle>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Primary,
    Danger,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Text {
    #[serde(rename = "plain_text")]
    PlainText { text: String },
    #[serde(rename = "mrkdwn")]
    Markdown { text: String },
}
