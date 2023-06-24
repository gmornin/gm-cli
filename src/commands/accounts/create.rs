use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::{V1All3, V1Response};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present, prompt_password_not_present};
use crate::traits::ConfigTriat;

const ARGS: &[&str] = &["username", "email", "password"];

pub fn create(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;

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
    prompt_not_present("Email", "email", &mut map);
    prompt_password_not_present("Password", "password", &mut map);

    let user = map.get("username").unwrap();
    if !user.contains(':') {
        error!("This does not seem to be a valid username string");
        return Err(CError::StringErr(format!("Invalid username string `{user}`")).into());
    }
    let (username, instance) = user.split_once(':').unwrap();
    let email = map.get("email").unwrap().to_string();
    let password = map.get("password").unwrap().to_string();

    let url = format!("{}/api/accounts/v1/create", instance);

    let body = V1All3 {
        email,
        username: username.to_string(),
        password,
    };

    let res: V1Response = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Failed to create account");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Created { id, token } => {
            info!("Account has been created");
            info!("Your user id is `{id}`");
            let account = AccountConfig {
                id,
                instance: instance.to_string(),
                token,
            };
            if let Err(e) = account.save() {
                error!("However, saving to file failed, you will not stay logged in");
                info!("You can still log in later using the account and password");
                return Err(e);
            } else {
                info!("Token and ID are saved, and you will stay logged in");
                info!("A verification email has been sent, verify your email address to gain more permissions")
            }
        }
        _ => unreachable!(),
    }
    Ok("Ran".to_string())
}
