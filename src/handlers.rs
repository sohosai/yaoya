use crate::model::Config;
use std::collections::HashMap;
use std::convert::Infallible;

pub async fn handle_interactive_components(
    _body: HashMap<String, String>,
    _config: Config,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("Hello"))
}
