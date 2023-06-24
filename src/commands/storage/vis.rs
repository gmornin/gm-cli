use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use goodmorning_bindings::services::v1::{
    ItemVisibility, V1PathOnly, V1PathVisibility, V1Response,
};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present};

const ARGS: &[&str] = &["path", "vis"];

pub fn vis(mut map: HashMap<String, String>, args: Vec<String>) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    prompt_not_present("Path", "path", &mut map);
    prompt_not_present(
        "Visibility (private, public, hidden, inherit)",
        "vis",
        &mut map,
    );

    let instance = map.get("instance").unwrap();
    let vis_str = map.get("vis").unwrap();
    let prefix = PathBuf::from(map.get("prefix").unwrap_or(&String::new()));
    let path = prefix.join(map.get("path").unwrap());

    if !path.has_root() {
        error!("User file paths must start with root `/`");
        return Err(CError::StrErr("invalid file path").into());
    }

    let path = path.to_str().unwrap().to_string();
    let token = map.get("token").unwrap().to_string();

    let vis = match vis_str.as_str() {
        "private" => ItemVisibility::Private,
        "public" => ItemVisibility::Public,
        "hidden" => ItemVisibility::Hidden,
        "inherit" => ItemVisibility::Private, // placeholder
        _ => return Err(CError::StringErr(format!("Invalid option `{vis_str}`")).into()),
    };

    let res = if vis_str.as_str() == "inherit" {
        let url = format!("{}/api/storage/v1/remove-visibility", instance,);

        let body = V1PathOnly { path, token };

        post(&url, body, map.contains_key("http"))?
    } else {
        let body = V1PathVisibility {
            path,
            visibility: vis,
            token,
        };

        let url = format!("{}/api/storage/v1/set-visibility", instance,);
        post(&url, body, map.contains_key("http"))?
    };

    match res {
        V1Response::Error { kind } => {
            error!("Visibility not changed");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::VisibilityChanged if vis_str.as_str() == "inherit" => {
            info!("Visibility reset");
        }
        V1Response::VisibilityChanged => {
            info!("Visibility changed to `{vis:?}`");
        }
        V1Response::NothingChanged => {
            warn!("Nothing changed");
        }
        _ => unreachable!(),
    }

    Ok(String::from("Finished"))
}
