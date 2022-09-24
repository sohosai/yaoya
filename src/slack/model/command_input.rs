use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandInputError {
    #[error("missing required field `{0}`")]
    MissingRequiredField(String),
}

pub struct CommandInput {
    token: String,
    command: String,
    text: String,
    response_url: String,
    trigger_id: String,
    user_id: String,
    user_name: String,
    channel_id: String,
    api_app_id: String,
}

impl TryFrom<HashMap<String, String>> for CommandInput {
    type Error = CommandInputError;
    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(CommandInput {
            token: get_field(&map, "token")?,
            command: get_field(&map, "command")?,
            text: get_field(&map, "text")?,
            response_url: get_field(&map, "response_url")?,
            trigger_id: get_field(&map, "trigger_id")?,
            user_id: get_field(&map, "user_id")?,
            user_name: get_field(&map, "user_name")?,
            channel_id: get_field(&map, "channel_id")?,
            api_app_id: get_field(&map, "api_app_id")?,
        })
    }
}

impl CommandInput {
    pub fn command(&self) -> String {
        self.command.to_string()
    }

    pub fn text(&self) -> String {
        self.text.to_string()
    }
}

fn get_field(map: &HashMap<String, String>, field: &str) -> Result<String, CommandInputError> {
    map.get(field)
        .ok_or(CommandInputError::MissingRequiredField(field.to_string()))
        .map(|s| s.to_string())
}
