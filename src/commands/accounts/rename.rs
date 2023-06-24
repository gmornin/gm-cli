use std::collections::HashMap;
use std::error::Error;

use goodmorning_bindings::services::v1::{V1RenameAccount, V1Response};
use log::*;

use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present};
use crate::{config::AccountConfig, functions::yes};

const ARGS: &[&str] = &["newname"];

pub fn rename(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    warn!("Your username will be changed");
    yes(&map);

    prompt_not_present("Your new username", "newname", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!("{}/api/accounts/v1/rename", instance);

    let new = map.get("newname").unwrap();
    let token = map.get("token").unwrap().to_string();

    let body = V1RenameAccount {
        token,
        new: new.to_string(),
    };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Failed to regenerate token");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Renamed => {
            info!("Rename successful");
            info!("Your new username is {}", new.to_lowercase());
        }
        _ => unreachable!(),
    }

    Ok(String::from("Renamed"))
}
