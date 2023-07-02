use std::collections::HashMap;
use std::error::Error;

use goodmorning_bindings::services::v1::{V1Response, V1SetStatus};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present};

const ARGS: &[&str] = &["status"];

pub fn status(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    prompt_not_present("Your new status", "status", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!("{}/api/accounts/v1/set-status", instance);

    let token = map.get("token").unwrap().to_string();
    let status = map.get("status").unwrap().to_string();
    if status.len() > 128 {
        error!("Exceeds maximum length (128)");
        return Err(CError::StrErr("exceeds maximum length").into());
    }

    let body = V1SetStatus { token, new: status };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Failed to change status");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::ProfileUpdated => {
            info!("Status update successful");
        }
        _ => unreachable!(),
    }

    Ok(String::from("Renamed"))
}
