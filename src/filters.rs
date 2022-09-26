mod verify;
use crate::handlers;
use crate::model::Config;
use verify::with_verify;
use warp::Filter;
use warp::{get, post};

use handlers::EmailVerificationOptions;

fn signup_command(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post()
        .and(warp::path("signup"))
        .and(with_verify(config.clone()))
        .and_then(move |body| handlers::handle_slash_commands(body, config.clone()))
}

fn interactive_components(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post()
        .and(warp::path("interactivity"))
        .and(with_verify(config.clone()))
        .and_then(move |body| handlers::handle_interactivity(body, config.clone()))
}

pub fn verify_email(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get()
        .and(warp::path("verify"))
        .and(warp::query::<EmailVerificationOptions>())
        .and_then(move |query| handlers::verify_email(query, config.clone()))
}

pub fn filter(
    config: &Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    signup_command(config.clone())
        .or(interactive_components(config.clone()))
        .or(verify_email(config.clone()))
        .with(warp::log("INFO"))
}
