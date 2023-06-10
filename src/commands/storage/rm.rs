use std::collections::HashMap;
use std::error::Error;

use goodmorning_bindings::services::v1::{V1PathOnly, V1Response};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{post, prompt_not_present};

pub fn rm(mut map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    prompt_not_present("Path", "path", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!("{}/api/services/v1/storage/delete", instance,);

    let path = map.get("path").unwrap().to_string();
    let token = map.get("token").unwrap().to_string();

    let body = V1PathOnly {
        path,
        token,
    };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("File not copied");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Deleted => {
            info!("Item has been deleted");
        }
        _ => unreachable!(),
    }

    Ok(String::from("Copied"))
}
