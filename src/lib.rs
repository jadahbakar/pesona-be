use std::error::Error;

use crate::utils::config::Config;

pub mod utils;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::new("./config/config.yaml")?;
    let db_opt = C
    
    Ok(())
}
