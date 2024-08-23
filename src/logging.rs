/// # Logging using syn_crabs
/// ### Multi-colored logging
/// 
/// Use `setup_logging` to setup logging
/// 
/// ```rust
/// use syn_crabs::setup_logging;
/// use log::{LevelFilter, debug error, info};
/// 
/// setup_logging().expect("Failed to setup logging");
/// ```
/// 
/// RUST_LOG environment variable sets level eg. Debug, Info, Warn, Error
/// 
/// 
use syn_crabs::setup_logging;

pub fn init_logging() {
    setup_logging().expect("Logging Init Failed");
    log::info!("Logging initialized.");
    log::debug!("{}", message);
    logging::error!("{}", message);
}
