use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub title: String,
    pub phone: String,
    pub real_name: String,
    pub real_name_normalized: String,
    pub display_name: String,
    pub display_name_normalized: String,
    pub fields: HashMap<String, HashMap<String, String>>,
    pub status_text: String,
    pub status_emoji: String,
    pub status_expiration: u64,
    pub avatar_hash: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_72: String,
    pub image_192: String,
    pub image_512: String,
}
