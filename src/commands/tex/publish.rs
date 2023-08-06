use crate::error::Error as CError;
use crate::functions::{post, prompt_not_present};
use crate::{config::AccountConfig, functions::map_args};
use goodmorning_bindings::services::v1::{V1Publish, V1Response};
use log::*;
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

const ARGS: &[&str] = &["path", "title", "desc"];

pub fn publish(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You must be logged in to view user files");
        return Err(CError::StrErr("not logged in").into());
    }

    prompt_not_present("Path (`/tex` omitted)", "path", &mut map);
    prompt_not_present("Title", "title", &mut map);
    prompt_not_present("Description", "desc", &mut map);

    let path = PathBuf::from(map.get("path").unwrap());
    if !path.has_root() {
        error!("User file paths must start with root `/`");
        return Err(CError::StrErr("invalid file path").into());
    }

    let path = path.to_str().unwrap().to_string();
    let title = map.get("title").unwrap().to_string();
    let desc = map.get("desc").unwrap().to_string();

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap().to_string();

    let url = format!("{instance}/api/tex/publish/v1/publish");
    let body = V1Publish {
        token,
        path,
        title,
        desc,
    };

    let res: V1Response = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("File not published");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::TexPublished { id } => {
            info!("Your published item has ID of {id}")
        }
        _ => unreachable!(),
    }

    Ok(String::from("Published"))
}
