use std::collections::HashMap;
use std::error::Error;

use goodmorning_bindings::services::v1::{V1FromTo, V1Response};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{post, prompt_not_present};

pub fn cp(mut map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    prompt_not_present("From", "from", &mut map);
    prompt_not_present("To", "to", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!(
        "{}/api/services/v1/storage/{}",
        instance,
        if map.contains_key("overwrite") {
            "copy-overwrite"
        } else {
            "copy"
        }
    );

    let from = map.get("from").unwrap().to_string();
    let to = map.get("to").unwrap().to_string();
    let from_user = map.get("user").unwrap_or(map.get("id").unwrap());
    let token = map.get("token").unwrap().to_string();

    let body = V1FromTo {
        from,
        to: to.clone(),
        from_userid: from_user.parse()?,
        token,
    };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("File not copied");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Copied => {
            info!("Item copied successfully");
            info!("The copied path is `{to}`");
        }
        _ => unreachable!(),
    }

    Ok(String::from("Copied"))
}
