use crate::model::Config;
use chrono::Utc;
use hmac_sha256::HMAC;
use thiserror::Error;
use warp::Filter;

pub fn with_verify(config: Config) -> warp::filters::BoxedFilter<((),)> {
    warp::header::headers_cloned()
        .and(warp::body::bytes())
        .and_then(move |headers, body| verify(headers, body, config.clone()))
        .boxed()
}

#[derive(Error, Debug)]
enum SignatureVerificationError {
    #[error("X-Slack-Request-Timestamp header is missing.")]
    MissingTimestamp,
    #[error("X-Slack-Signature header is invalid.")]
    MissingSignature,
    #[error("Parse error")]
    ParseError,
    #[error("Invalid signature")]
    InvalidSignature,
}

impl warp::reject::Reject for SignatureVerificationError {}

#[derive(Debug)]
struct SignatureHeaderValues {
    pub x_slack_request_timestamp: String,
    pub x_slack_signature: String,
}

async fn verify(
    headers: warp::http::HeaderMap<warp::http::HeaderValue>,
    body: bytes::Bytes,
    config: Config,
) -> Result<(), warp::Rejection> {
    let header_values = match extract_signature_headers(&headers) {
        Ok(header_values) => header_values,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let timestamp = header_values.x_slack_request_timestamp;
    let computed_signature = match compute_signature(&timestamp, body, &config.slack_signing_secret)
    {
        Ok(computed_signature) => computed_signature,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let signature = header_values.x_slack_signature;

    if signature != format!("v0={}", computed_signature) {
        return Err(warp::reject::custom(
            SignatureVerificationError::InvalidSignature,
        ));
    }
    Ok(())
}

fn extract_signature_headers(
    headers: &warp::http::HeaderMap<warp::http::HeaderValue>,
) -> Result<SignatureHeaderValues, SignatureVerificationError> {
    let x_slack_request_timestamp = headers
        .get("X-Slack-Request-Timestamp")
        .ok_or(SignatureVerificationError::MissingTimestamp)?;
    let x_slack_signature = headers
        .get("X-Slack-Signature")
        .ok_or(SignatureVerificationError::MissingSignature)?;

    Ok(SignatureHeaderValues {
        x_slack_request_timestamp: x_slack_request_timestamp.to_str().unwrap().to_string(),
        x_slack_signature: x_slack_signature.to_str().unwrap().to_string(),
    })
}

fn compute_signature(
    timestamp: &str,
    body: bytes::Bytes,
    secret: &str,
) -> Result<String, SignatureVerificationError> {
    let timestamp_int = match timestamp.parse::<i64>() {
        Ok(timestamp_int) => timestamp_int,
        Err(e) => {
            return {
                error!("Too old signature given. Someone is trying to replay the request.");
                Err(SignatureVerificationError::InvalidSignature)
            }
        }
    };
    let current_timestamp = Utc::now().timestamp();

    // Token issued 5minutes ago or before is invalid because it may be a replay attack.
    if current_timestamp - timestamp_int > 60 * 5 {
        return Err(SignatureVerificationError::InvalidSignature);
    }

    let request_body =
        String::from_utf8(body.to_vec()).map_err(|_| SignatureVerificationError::ParseError)?;

    let signature_base_string = format!("v0:{}:{}", timestamp, request_body);
    let signature = HMAC::mac(signature_base_string, secret);
    let signature_hex = hex::encode(signature);

    Ok(signature_hex)
}
