use std::{error::Error, collections::HashMap, fs};

use log::*;

use crate::error::Error as CError;
use crate::{config::AccountConfig, traits::ConfigTriat};

pub fn logout(_map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    warn!("You are going to be logged out");

    let path = AccountConfig::path();

    match fs::remove_file(&path) {
        Ok(_) => info!("Any account login info has been deleted"),
        Err(e) => {
            error!("Could not delete file at `{path:?}`");
            return Err(CError::StringErr(e.to_string()).into());
        }
    }

    Ok(String::from("Deleted"))
}
