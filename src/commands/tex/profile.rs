use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::V1Response;
use log::*;

use crate::error::Error as CError;
use crate::functions::{display_profile, get, map_args, prompt_not_present};

const ARGS: &[&str] = &["username"];

pub fn profile(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;

    prompt_not_present(
        "Username (e.g. username:instance.com)",
        "username",
        &mut map,
    );

    let username = map.get("username").unwrap();

    if !username.contains(':') {
        error!("This does not seem to be a valid username string");
        return Err(CError::StringErr(format!("Invalid username string `{username}`")).into());
    }
    let (username, instance) = username.split_once(':').unwrap();

    let url = format!("{}/api/generic/v1/profile/name/{username}", instance);

    let res: V1Response = get(&url, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Cannot load profile");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Profile { profile, account } => {
            info!("Recieved profile");
            println!("{}", display_profile(&profile, &account, instance));
        }
        _ => unreachable!(),
    }
    Ok("Ran".to_string())
}
