//! # Inferno Inference Engine
//! ### Model orchestration and chaining using Rust
//! 

mod errors;
mod logging; 

use crate::logging::init_logging;
use crate::errors::CustomError;
use std::result::Result;

pub fn main() -> Result<(), CustomError> {
    // Initialize logging
    init_logging()
        .Ok()
        .expect(|e| CustomError::LoggingError(format!("Failed to setup logging: {}", e)))?;

    // Use the log macros provided by the log crate
    logging::info!("Welcome to Inferno! Binary executed successfully.");
    logging::error!("THIS IS AN ERROR SAMPLE.");

    Ok(())
}
