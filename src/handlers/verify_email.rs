use crate::model::username::{Username, UsernameError};
use crate::model::Config;
use crate::slack::model::{
    message::{ActionElement, Block, ButtonStyle, Text},
    Message,
};
use crate::slack::post_message::post_message;
use crate::verify_token;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailVerificationOptions {
    pub token: String,
    pub email: String,
    pub iat: u64,
    pub user_id: String,
    pub real_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum InteractiveComponentValue {
    IsRealnameCorrectPromptAnswer {
        token: String,
        email: String,
        iat: u64,
        user_id: String,
        real_name: String,
    },
}

pub async fn verify_email(
    params: EmailVerificationOptions,
    config: Config,
) -> Result<impl warp::Reply, Infallible> {
    info!(
        "Email {} is verified for user {}({})",
        params.email, params.real_name, params.user_id
    );

    if verify_token(&params.token, params.iat, &params.email, &config).is_err(){
        return Ok(
            warp::reply::html(generate_html("Error", "Invalid token Bad request)", "アクセストークンが無効であるか、失効しています。メールアドレスの確認は、メールの送信から5分以内に行なってください。"))
        );
    }

    let message = match Username::try_from(params.real_name.as_str()) {
        Ok(username) => match username_to_confirmation_message(username, &params) {
            Ok(msg) => msg,
            Err(res) => return Ok(res),
        },
        Err(e) => username_error_to_error_message(e, &params),
    };

    if let Err(e) = post_message(&config.slack_bot_token, &params.user_id, message).await {
        error!("Failed to post message: {}", e);
        return Ok(
            warp::reply::html(generate_html("Error", "Failed to post message.(Internal server error)", "サーバ内部でのエラーです。情報メディアシステム局までお問い合わせください。ご不便をおかけして申し訳ありません。"))
        );
    }

    Ok(warp::reply::html(generate_html(
        "確認完了",
        "メールアドレスが確認されました。",
        "SlackのDMから手続きを進めてください。",
    )))
}

fn generate_html(title: &str, subtitle: &str, desctiption: &str) -> String {
    format!(
        r#"
    <!doctype html>
    <html lang="ja">
    <head>
        <meta charset="utf-8">
        <title>yaoya at negicloud</title>
    </head>
    <body>
        <h1>{}</h1>
        <p>{}</p>
        <p>{}</p>
    </html>
    "#,
        title, subtitle, desctiption
    )
}

pub fn username_error_to_error_message(
    e: UsernameError,
    params: &EmailVerificationOptions,
) -> Message // Title,subtitle,description{{
{
    let text = match e{
        UsernameError::InvalidCharacters => format!("氏名「{}」には使用できない文字が含まれています。命名規則に準拠すれば、氏名は半角英数字のみになります。氏名の設定を変更、はじめからやり直してください。\n よくあるミス: 全角文字を含む、半角スペースを含むなど",params.real_name),
        UsernameError::Malformed(_,_,_,_)=> format!("氏名「{}」は形式が不正です。受け付けられるユーザ名は、たとえば'21jsysItoYusei'のような形式です。氏名の設定を変更後、はじめからやり直してください。",params.real_name),
        UsernameError::MissingYear=> format!("氏名「{}」に年次が含まれていません。たとえば'21jsysItoYusei'のように、入学年次の西暦下2桁を含めてください。氏名の設定を変更後、はじめからやり直してください。",params.real_name),
        UsernameError::UnknownDepartment(d)=> format!("氏名「{}」に規定外の局/組織名「{}」が含まれています。",params.real_name,d)
    };

    Message::Text(text)
}

pub fn username_to_confirmation_message(
    username: Username,
    params: &EmailVerificationOptions,
) -> Result<Message, warp::reply::Html<String>> {
    let message =format!("ユーザ名{}によれば、あなたは{}所属(20{}年入学)の{} {}さん(姓,名の順)です。間違いはありませんか?",&params.real_name,&username.department.to_japanese(),&username.year,&username.family_name,&username.given_name);

    let continue_action_id = format!("{}-{}-continue", params.user_id, params.iat);
    let cancel_action_id = format!("{}-{}-cancel", params.user_id, params.iat);
    let value = InteractiveComponentValue::IsRealnameCorrectPromptAnswer {
        token: params.token.to_string(),
        email: params.email.to_string(),
        iat: params.iat,
        user_id: params.user_id.to_string(),
        real_name: params.real_name.to_string(),
    };

    let value = match serde_json::to_string(&value) {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to serialize value: {}", e);
            return Err(
            warp::reply::html(generate_html("Error", "Failed to serialize value.(Internal server error)", "サーバ内部でのエラーです。情報メディアシステム局までお問い合わせください。ご不便をおかけして申し訳ありません。"))
            );
        }
    };

    let blocks = vec![
        Block::Section {
            text: Text::PlainText { text: message },
        },
        Block::Actions {
            elements: vec![
                ActionElement::Button {
                    action_id: cancel_action_id,
                    text: Text::PlainText {
                        text: "訂正する".to_string(),
                    },
                    value: value.clone(),
                    style: None,
                },
                ActionElement::Button {
                    action_id: continue_action_id,
                    text: Text::PlainText {
                        text: "確認して続ける".to_string(),
                    },
                    value,
                    style: Some(ButtonStyle::Primary),
                },
            ],
        },
    ];

    Ok(Message::Blocks(blocks))
}
