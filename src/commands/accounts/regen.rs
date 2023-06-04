use std::collections::HashMap;
use std::error::Error;

use goodmorning_bindings::services::v1::{V1PasswordId, V1Response};
use log::*;

use crate::error::Error as CError;
use crate::functions::post;
use crate::traits::ConfigTriat;
use crate::{
    config::AccountConfig,
    functions::{prompt_password_not_present, yes},
};

pub fn regen(mut map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    warn!("Proceeding will invalidate all your other logins");
    yes(&map);

    prompt_password_not_present("Enter your password", "password", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!("{}/api/services/v1/account/regeneratetoken", instance);

    let id = map.get("id").unwrap().to_string();
    let password = map.get("password").unwrap().to_string();

    let body = V1PasswordId {
        identifier: id.clone(),
        identifier_type: goodmorning_bindings::services::v1::V1IdentifierType::Id,
        password,
    };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Failed to regenerate token");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::RegenerateToken { token } => {
            let account = AccountConfig {
                id,
                instance: instance.to_string(),
                token,
            };

            if let Err(e) = account.save() {
                error!("However, saving to file failed, therefore you will be logged out");
                info!("You can still log in later using the account and password");
                return Err(e);
            } else {
                warn!("Your account has been logged out in all other devices");
                info!("New token saved, and you will stay logged in");
            }
        }
        _ => unreachable!(),
    }

    Ok(String::from("Saved"))
}
