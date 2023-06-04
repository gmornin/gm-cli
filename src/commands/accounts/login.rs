use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::{V1PasswordId, V1Response};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{post, prompt_not_present, prompt_password_not_present};
use crate::traits::ConfigTriat;

pub fn login(mut map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    warn!(
        "Your account ID and token will be stored in {:?}",
        AccountConfig::path()
    );
    warn!("This means that anyone with permission to see that file will have access to your token, thus your account");
    warn!("If you do not wish that to happen, exit with Ctrl + C");

    info!("If you have any other accounts on this device, continuing will overwrite that account's login");

    prompt_not_present(
        "Username (e.g. username:instance.com)",
        "username",
        &mut map,
    );
    prompt_password_not_present("Password", "password", &mut map);

    let user = map.get("username").unwrap().to_string();
    if !user.contains(':') {
        error!("This does not seem to be a valid username string");
        return Err(CError::StringErr(format!("Invalid username string `{user}`")).into());
    }
    let (username, instance) = user.split_once(':').unwrap();
    let password = map.get("password").unwrap().to_string();

    let url = format!("{}/api/services/v1/account/login", instance);

    let body = V1PasswordId {
        identifier: username.to_string(),
        identifier_type: goodmorning_bindings::services::v1::V1IdentifierType::Username,
        password,
    };

    let res: V1Response = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Failed to create account");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Login { token, id } => {
            let account = AccountConfig {
                id,
                instance: instance.to_string(),
                token,
            };

            info!("Login successful");

            if let Err(e) = account.save() {
                error!("However, saving to file failed, you will not stay logged in");
                return Err(e);
            } else {
                info!("Token and ID is saved, and you will stay logged in")
            }
        }
        _ => unreachable!(),
    }
    Ok("Ran".to_string())
}
