#[macro_use]
extern crate log;

use anyhow::Result;
mod filters;
mod handlers;
mod model;
mod slack;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up...");

    println!("Hello, world!");

    info!("Listening on port 3030");

    let config = match envy::from_env::<model::Config>() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    warp::serve(filters::filter(&config))
        .run(([0, 0, 0, 0], 3030))
        .await;
    Ok(())
}
