use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::V1Response;
use log::*;

use crate::error::Error as CError;
use crate::functions::{display_publish_item, get, map_args, prompt_not_present};

const ARGS: &[&str] = &["username", "page", "per_page"];

pub fn publishes(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;

    let per_page = map
        .get("per_page")
        .unwrap_or(&"10".to_string())
        .parse::<u64>()
        .unwrap();
    prompt_not_present(
        "Username (e.g. username:instance.com)",
        "username",
        &mut map,
    );
    prompt_not_present(
        &format!("Page ({per_page} items per page)"),
        "page",
        &mut map,
    );

    let username = map.get("username").unwrap();
    let page = map.get("page").unwrap();

    if !username.contains(':') {
        error!("This does not seem to be a valid username string");
        return Err(CError::StringErr(format!("Invalid username string `{username}`")).into());
    }
    let (username, instance) = username.split_once(':').unwrap();

    let url = format!(
        "{}/api/publish/v1/publishes/name/{username}?page={page}&page_size={per_page}",
        instance
    );

    let res: V1Response = get(&url, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Cannot load profile");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::TexUserPublishes { items } => {
            info!("Recieved items");
            if items.is_empty() {
                info!("Items empty")
            } else {
                println!(
                    "{}",
                    items
                        .iter()
                        .map(display_publish_item)
                        .collect::<Vec<String>>()
                        .join("\n---\n")
                );
            }
        }
        _ => unreachable!(),
    }
    Ok("Ran".to_string())
}
