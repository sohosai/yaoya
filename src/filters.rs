mod verify;
use crate::handlers;
use crate::model::Config;
use verify::with_verify;
use warp::post;
use warp::Filter;

fn signup_command(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post()
        .and(warp::path("signup"))
        .and(with_verify(config.clone()))
        .and(warp::body::form())
        .and_then(move |_, body| handlers::handle_interactive_components(body, config.clone()))
}

fn interactive_components(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post()
        .and(warp::path("interactive-components"))
        .and(with_verify(config.clone()))
        .and(warp::body::form())
        .and_then(move |_, body| handlers::handle_interactive_components(body, config.clone()))
}

pub fn filter(
    config: &Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    signup_command(config.clone())
        .or(interactive_components(config.clone()))
        .with(warp::log("INFO"))
}
