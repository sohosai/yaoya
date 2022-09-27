use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Error while sending requewst {0}")]
    Request(#[from] reqwest::Error),
    #[error("Error while parsing response {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Error status is {0} is returned with message {1}")]
    SlackApi(u16, String),
}
