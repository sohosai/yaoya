mod model;
use crate::model::Config;
use std::convert::Infallible;


pub async fn handle_interactivity(
 _body: String,
 _config: Config,
) -> Result<impl warp::Reply, Infallible> {
 
 Ok(warp::reply::html("Hello"))
}
