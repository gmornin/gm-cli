use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, prompt_not_present, post};

use goodmorning_bindings::services::v1::{V1Response, V1Compile, FromFormat, ToFormat};
use log::*;

const ARGS: &[&str] = &["path", "from", "to"];

pub fn compile(mut map: HashMap<String, String>, args: Vec<String>) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You must be logged in to view user files");
        return Err(CError::StrErr("not logged in").into());
    }

    prompt_not_present("Path (`/tex` omitted)", "path", &mut map);
    prompt_not_present("From format", "from", &mut map);
    prompt_not_present("To format", "to", &mut map);

    let path = PathBuf::from(map.get("path").unwrap());
    if !path.has_root() {
        error!("User file paths must start with root `/`");
        return Err(CError::StrErr("invalid file path").into());
    }

    let path = path.to_str().unwrap().to_string();

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap().to_string();

    let url = format!("{instance}/api/tex/compile/v1/simple");

    let from = match map.get("from").unwrap().as_str() {
        "md" | "markdown" => FromFormat::Markdown,
        _ => {
            error!("Valid formats are: `markdown`");
            return Err(CError::StrErr("invalid from format").into());
        }
    };
    let to = match map.get("to").unwrap().as_str() {
        "html" => ToFormat::Html,
        _ => {
            error!("Valid formats are: `html`");
            return Err(CError::StrErr("invalid to format").into());
        }
    };

    let body = V1Compile {
        path,
        token,
        from,
        to,
    };

    let res: V1Response = post(&url, body,  map.contains_key("http"))?;

    match res {
        V1Response::Compiled { id, newpath, message } => {
            info!("Compile success");
            info!("New path: {newpath}");
            info!("Job ID: {id}");
            if !message.is_empty() {
                info!("Returned with message `{message}`");
            }
        },
        V1Response::Error { kind } => {
            error!("Unable to display file content");
            return Err(kind.into());
        }
        _ => unreachable!(),
    }

    Ok(String::from("Finished"))
}
