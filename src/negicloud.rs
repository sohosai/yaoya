use crate::model::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserParams {
    pub userid: String,
    pub password: String, // Leave empty to send welcome email
    pub email: String,    // Required if password empty
    pub groups: Vec<String>,
}

impl RegisterUserParams {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::from([
            ("userid".to_string(), self.userid.to_string().to_owned()),
            ("email".to_string(), self.email.to_string()),
            ("password".to_string(), self.password.to_string()),
        ]);

        for group in self.groups.iter() {
            map.insert("groups[]".to_string(), group.to_string());
        }

        map
    }
}

#[derive(Debug, Error)]
pub enum RegisterUserError {
    #[error("API Request error. {0}")]
    ApiRequestError(#[from] reqwest::Error),
    #[error("Negicloud responded with {0}. Reason: {1}")]
    NegicloudError(u16, String),
    #[error("Duplicate")]
    Duplicate,
}

pub async fn register_user(
    config: &Config,
    params: RegisterUserParams,
) -> Result<(), RegisterUserError> {
    let s = format!("https://{}/ocs/v1.php/cloud/users", config.negicloud_host);

    info!("{}", s);

    let client = reqwest::Client::new();
    let res = client
        .post(&s)
        .basic_auth(
            &config.negicloud_admin_user,
            Some(&config.negicloud_admin_password),
        )
        .form(&params.to_hashmap())
        .header("OCS-APIRequest", "true")
        .send()
        .await?;

    if res.status().is_success() {
        let text = res
            .text()
            .await
            .unwrap_or("Failed to stringify".to_string());
        if text.contains("<statuscode>102</statuscode>") {
            //TODO: PARSE XML
            error!("Duplicate. {}", text);
            return Err(RegisterUserError::Duplicate);
        }

        info!("Negicloud API request success. text: {}", text);
        Ok(())
    } else {
        Err(RegisterUserError::NegicloudError(
            res.status().as_u16(),
            res.text()
                .await
                .unwrap_or("<Failed to stringify>".to_string()),
        ))
    }
}
