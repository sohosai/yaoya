use crate::model::Config;
use crate::slack::model::CommandInput;
use crate::slack::get_profile::get_profile;

use crate::model::email::{UniversityEmailAddress,UniveristyEmailAddressError};

use serde::Serialize;

use thiserror::Error;
use sendgrid::{Mail, SGClient};
use sendgrid::SendgridError;
use chrono::Utc;

#[derive(Debug, Error)]
pub enum SignupError {
    #[error("Slack api error {0}")]
    SlackError(#[from] crate::slack::model::Error),
}

impl Into<warp::http::StatusCode> for SignupError {
    fn into(self) -> warp::http::StatusCode {
        warp::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

enum EmailSourceType{
    Argument,
    Registerd
}

pub async fn signup(input: &CommandInput, config: &Config) -> Result<impl Serialize, SignupError> {

    info!("Signup command invoked");
    
    let email = input.text.trim();
    let (email,source) = match UniversityEmailAddress::try_from(email){
        Ok(email) => (email,EmailSourceType::Argument),
        Err(UniveristyEmailAddressError::IsNotUniversityEmail) => return Ok("与えられたメールアドレスは有効な形式のsアドではありません。 `\\signup sXXXXXXX@s.tsukuba.ac.jp` のようにしてご自身のsアドを指定してください。".to_string()),
        Err(UniveristyEmailAddressError::MalformedEMailAdderess(_)) => {
        // Input is none or malformed.
            let user = get_profile(&input.user_id, config).await?;
            match UniversityEmailAddress::try_from(user.email.as_str()){
                Ok(email) => (email,EmailSourceType::Registerd),
                Err(UniveristyEmailAddressError::IsNotUniversityEmail) => return Ok("Slackに登録されたメールアドレスはsアドではありません。`\\signup sXXXXXXX@s.tsukuba.ac.jp` のようにしてご自身のsアドを指定してください。".to_string()),
                Err(UniveristyEmailAddressError::MalformedEMailAdderess(_)) => {
                    error!("Registerd email is malformed: {}", user.email);
                    return Ok("Slackに登録されたメールアドレスの形式が不正です".to_string())
                },
            }
        },
    };

    
    if let Err(e) = send_verification_email(&config,&email.to_string()).await{
        error!("Failed to send verification email: {}",e);
        return Ok("メールアドレスの確認メールの送信に失敗しました。しばらくしてからもう一度お試しください。繰り返し試してもうまくいかない場合は、情報メディアシステム局までお尋ねください。".to_string());
    }

    let response = match source{
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


pub async fn send_verification_email(config: &Config,email: &str)->Result<(),SendgridError>{

    let iat = Utc::now().timestamp().to_string();
    let token_basestring = format!("{}-{}-{}",iat,email, config.verify_salt);
    let token = hmac_sha256::Hash::hash(token_basestring.as_bytes());
    let token = hex::encode(token);


    let mail_content = format!("negicloudご利用者様\n 日頃よりnegicloudご利用いただきありがとうございます。以下のリンクに移動して、メールアドレスを確認してください。\n {}verify?token={}&email={}&iat={}",config.my_baseurl,token,email,iat);
    let mail = Mail::new()
        .add_from("jsys@sohosai.com")
        .add_text(&mail_content)
        .add_subject("Hello")
        .add_to((email, email).into());

    SGClient::new(&config.sendgrid_token)
        .send(mail)?;
    Ok(())
}
