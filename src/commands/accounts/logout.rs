use std::{collections::HashMap, error::Error};

use log::*;

use crate::error::Error as CError;
use crate::functions::yes;
use crate::{config::AccountConfig, traits::ConfigTriat};

pub fn logout(map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    if !AccountConfig::is_loggedin_map(&map) {
        info!("You don't seemed to be logged in");
        return Ok(String::from("Nothing changed"));
    }

    warn!("You are going to be logged out");

    yes(&map);

    let path = AccountConfig::path();

    match AccountConfig::clear() {
        Ok(_) => info!("Any account login info has been deleted"),
        Err(e) => {
            error!("Could not delete file at `{path:?}`");
            return Err(CError::StringErr(e.to_string()).into());
        }
    }

    Ok(String::from("Deleted"))
}
