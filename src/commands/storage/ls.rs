use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{diritem_tostring, get, prompt_not_present};

use goodmorning_bindings::services::v1::V1Response;
use log::*;

pub fn ls(mut map: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You must be logged in to view user files");
        return Err(CError::StrErr("not logged in").into());
    }

    prompt_not_present("Path", "path", &mut map);

    let path = map.get("path").unwrap();

    if !path.starts_with('/') {
        error!("User file paths must start with root `/`");
        return Err(CError::StrErr("invalid file path").into());
    }

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap();

    let url = format!(
        "{instance}/api/services/v1/storage/diritems/{token}/{}",
        &path[1..]
    );

    let res: V1Response = get(&url, map.contains_key("http"))?;

    match res {
        V1Response::DirContent { mut content } => {
            let longest_size = if content.is_empty() {
                0
            } else {
                content.sort_by(|this, other| this.name.cmp(&other.name));
                content
                    .iter()
                    .max_by(|this, other| this.size.cmp(&other.size))
                    .unwrap()
                    .size
                    .to_string()
                    .len()
            };

            println!("---");
            println!("{} items", content.len());
            content
                .iter()
                .for_each(|item| println!("{}", diritem_tostring(item, longest_size, &PathBuf::from(path))));
            println!("---");
        }
        V1Response::Error { kind } => {
            error!("Unable to display file content");
            return Err(kind.into());
        }
        _ => unreachable!(),
    }

    Ok(String::from("Finished"))
}