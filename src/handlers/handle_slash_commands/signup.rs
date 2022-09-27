use crate::model::Config;
use crate::slack::get_profile::get_profile;
use crate::slack::model::CommandInput;

use crate::model::email::{UniveristyEmailAddressError, UniversityEmailAddress};

use serde::Serialize;

use chrono::Utc;
use sendgrid::SendgridError;
use sendgrid::{Mail, SGClient};
use thiserror::Error;
use urlencoding::encode;

#[derive(Debug, Error)]
pub enum SignupError {
    #[error("Slack api error {0}")]
    SlackError(#[from] crate::slack::model::Error),
}

impl From<SignupError> for warp::http::StatusCode {
    fn from(_: SignupError) -> warp::http::StatusCode {
        warp::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

enum EmailSourceType {
    Argument,
    Registerd,
}

pub async fn signup(input: &CommandInput, config: &Config) -> Result<impl Serialize, SignupError> {
    info!("Signup command invoked");

    let user = get_profile(&input.user_id, config).await?;
    let email = input.text.trim();
    let (email,source) = match UniversityEmailAddress::try_from(email){
        Ok(email) => (email,EmailSourceType::Argument),
        Err(UniveristyEmailAddressError::IsNotUniversityEmail) => return Ok("与えられたメールアドレスは有効な形式のsアドではありません。 `/signup sXXXXXXX@s.tsukuba.ac.jp` のようにしてご自身のsアドを指定してください。".to_string()),
        Err(UniveristyEmailAddressError::MalformedEMailAdderess(_)) => {
        // Input is none or malformed.
            info!("Email is not given in argument. Fallback to the registerd email address.");

            match UniversityEmailAddress::try_from(user.email.as_str()){
                Ok(email) => (email,EmailSourceType::Registerd),
                Err(UniveristyEmailAddressError::IsNotUniversityEmail) => return Ok("Slackに登録されたメールアドレスはsアドではありません。`/signup sXXXXXXX@s.tsukuba.ac.jp` のようにしてご自身のsアドを指定してください。".to_string()),
                Err(UniveristyEmailAddressError::MalformedEMailAdderess(_)) => {
                    error!("Registerd email is malformed: {}", user.email);
                    return Ok("Slackに登録されたメールアドレスの形式が不正です".to_string())
                },
            }
        },
    };

    if let Err(e) =
        send_verification_email(config, &email.to_string(), &user.real_name, &input.user_id).await
    {
        error!("Failed to send verification email: {}", e);
        return Ok("メールアドレスの確認メールの送信に失敗しました。しばらくしてからもう一度お試しください。繰り返し試してもうまくいかない場合は、情報メディアシステム局までお尋ねください。".to_string());
    }

    let response = match source {
        EmailSourceType::Argument => {
            info!("Signing up with argument email address");
            format!("与えられたメールアドレス{}に確認メールを送信しました。メールに記載されたリンクをクリックしてください。",email.to_string())
        }
        EmailSourceType::Registerd => {
            info!("Signing up with registerd email address");
            format!("Slackに登録されたメールアドレス{}に確認メールを送信しました。メールに記載されたリンクをクリックしてください。",email.to_string())
        }
    };

    Ok(response)
}

pub async fn send_verification_email(
    config: &Config,
    email: &str,
    real_name: &str,
    user_id: &str,
) -> Result<(), SendgridError> {
    let iat = Utc::now().timestamp().to_string();
    let token_basestring = format!("{}-{}-{}", iat, email, config.verify_salt);
    let token = hmac_sha256::Hash::hash(token_basestring.as_bytes());
    let token = hex::encode(token);

    let email_encoded = encode(email).to_string();
    let url = format!(
        "{}verify?token={}&email={}&iat={}&user_id={}&real_name={}",
        config.my_baseurl, token, email_encoded, iat, user_id, real_name
    );

    let mail_content = format!("negicloudご利用者様\n 日頃よりnegicloudご利用いただきありがとうございます。以下のリンクに移動して、メールアドレスを確認してください。\n {}",url);
    let mail = Mail::new()
        .add_from(&config.email_from)
        .add_text(&mail_content)
        .add_subject("Hello")
        .add_to((email, email).into());

    SGClient::new(&config.sendgrid_token).send(mail).await?;
    Ok(())
}
