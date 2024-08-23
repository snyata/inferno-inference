mod errors;

use crate::errors::CustomError;
use std::result::Result;
use syn_crabs::setup_logging

// Initialize Logging


pub fn main() -> Result<(), CustomError> {
    setup_logging()?;

    log::info!("Welcome to Inferno! Binary executed successfully.");
    log::error!("THIS IS AN ERROR SAMPLE.");

    Ok(())
}
