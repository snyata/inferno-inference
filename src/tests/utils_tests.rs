#[test]
fn logging_initialization_success() {
    use syn_crabs::{ setup_logging, init_logging };
    use crate::init_logging;

    const log: log::Logger = setup_logging().expect("Failed to setup logging");

    init_logging();

    // Assuming the setup_logging function sets the log level to Info
    assert!(log::log_enabled!(log::Level::Info));
    assert!(log::log_enabled!(log::Level::Debug));
    assert!(log::log_enabled!(log::Level::Error));
}