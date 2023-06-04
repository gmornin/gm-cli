use std::collections::HashMap;
use std::error::Error;

use goodmorning_bindings::services::v1::{V1Response, V1TokenOnly};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{post, type_yes};
use crate::traits::ConfigTriat;

pub fn delete(map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    warn!("Proceeding wipe your account from existence");
    type_yes(&map);

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap().to_string();
    let url = format!("{}/api/services/v1/account/delete", instance);

    let body = V1TokenOnly { token };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Failed to regenerate token");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Deleted => {
            info!("Your account has been successfully deleted");

            if let Err(e) = AccountConfig::clear() {
                error!(
                    "However, saving to file failed, therefore you appear to still be logged on"
                );
                warn!("This appeared-to-be-logged-on is only client side, but the details are invalidated");
                return Err(e);
            } else {
                warn!("Your account has been deleted from the server");
            }
        }
        _ => unreachable!(),
    }

    Ok(String::from("Saved"))
}
