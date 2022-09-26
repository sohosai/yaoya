use crate::model::Config;
use std::convert::Infallible;

mod handle_slash_commands;
pub use handle_slash_commands::handle_slash_commands;

mod verify_email;
pub use verify_email::{verify_email, EmailVerificationOptions};

pub async fn handle_interactive_components(
    _body: String,
    _config: Config,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("Hello"))
}
