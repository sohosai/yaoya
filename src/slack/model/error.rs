use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error{
    #[error("Error while sending requewst {0}")] 
    RequestError(#[from] reqwest::Error),
    #[error("Error while parsing response {0}")]
    ParseError(#[from] serde_urlencoded::de::Error)
}
