mod arcdps;
mod executor;
mod exports;
mod logging;
mod music;
mod pubsub;
use std::env;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    logging::setup();
    logging::info(String::from("Setup logging"));
    let path = env::current_dir()?;
    logging::info(format!("The current directory is {}", path.display()));
    executor::setup();
    pubsub::setup();
    Ok(())
}

fn release() {
    pubsub::teardown();
    executor::teardown();
}
