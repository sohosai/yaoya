#[macro_use]
extern crate log;

use anyhow::Result;
mod filters;
mod handlers;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up...");

    println!("Hello, world!");

    info!("Listening on port 3030");

    warp::serve(filters::filter(&config))
        .run(([0, 0, 0, 0], 3030))
        .await;
    Ok(())
}
