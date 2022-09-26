use  crate::slack::model::message::{ActionElement, Block, Text};
use crate::model::Config;
use serde::{Serialize, Deserialize};
use std::convert::Infallible;

#[derive(Serialize, Deserialize,Debug)]
pub struct EmailVerificationOptions{
    pub text: Text,
    pub value: String,
}

pub async fn verify_email(params: EmailVerificationOptions,config: Config)-> Result<impl warp::Reply, Infallible>{
 let blocks = vec![
     Block::Section {
         text: Text::PlainText {
             text: "Hello World".to_string(),
         },
     },
     Block::Actions {
         elements: vec![
             ActionElement::Button {
                 action_id: "my-action-id-yes".to_string(),
                 text: Text::PlainText {
                     text: "確認して続ける".to_string(),
                 },
                 value: "clicked yes".to_string(),
             },
             ActionElement::Button {
                 action_id: "my-action-id-no".to_string(),
                 text: Text::PlainText {
                     text: "訂正する".to_string(),
                 },
                 value: "clicked yes".to_string(),
             },
         ],
     },
 ];

 let msg = Message::Blocks { blocks,token,channel };
 
 Ok(warp::reply::html("Hello"))
}
