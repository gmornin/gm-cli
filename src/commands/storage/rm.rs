use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use goodmorning_bindings::services::v1::{V1PathOnly, V1Response};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present};

const ARGS: &[&str] = &["path"];

pub fn rm(mut map: HashMap<String, String>, args: Vec<String>) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    prompt_not_present("Path", "path", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!("{}/api/storage/v1/delete", instance,);

    let prefix = PathBuf::from(map.get("prefix").unwrap_or(&String::new()));
    let path = prefix.join(map.get("path").unwrap());

    if !path.has_root() {
        error!("User file paths must start with root `/`");
        return Err(CError::StrErr("invalid file path").into());
    }

    let path = path.to_str().unwrap().to_string();
    let token = map.get("token").unwrap().to_string();

    let body = V1PathOnly { path, token };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("File not deleted");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Deleted => {
            info!("Item has been deleted");
        }
        _ => unreachable!(),
    }

    Ok(String::from("Copied"))
}
