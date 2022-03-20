use env_logger;
use log::{debug, error, info, warn};
use std::env;

pub fn logaaa() {
    println!("Hello, world!");
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    debug!("debugです");
    info!("infoです");
    warn!("warnです");
    error!("errorです");
}
