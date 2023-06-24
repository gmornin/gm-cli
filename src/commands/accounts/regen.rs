use std::collections::HashMap;
use std::error::Error;

use goodmorning_bindings::services::v1::{V1PasswordId, V1Response};
use log::*;

use crate::error::Error as CError;
use crate::functions::{map_args, post};
use crate::traits::ConfigTriat;
use crate::{
    config::AccountConfig,
    functions::{prompt_password_not_present, yes},
};

const ARGS: &[&str] = &["password"];

pub fn regen(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    warn!("Proceeding will invalidate all your other logins");
    yes(&map);

    prompt_password_not_present("Enter your password", "password", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!("{}/api/accounts/v1/regeneratetoken", instance);

    let id = map.get("id").unwrap().parse::<i64>()?;
    let password = map.get("password").unwrap().to_string();

    let body = V1PasswordId {
        identifier: id.to_string(),
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
