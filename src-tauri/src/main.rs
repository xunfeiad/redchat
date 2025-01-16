// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use error::{Error, Result};
fn main() -> Result<(), Error> {
    anycapture_lib::run()?;
    Ok(())
}
