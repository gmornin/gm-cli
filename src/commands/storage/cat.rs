use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

use std::fs::OpenOptions;
use std::io::Write;

use goodmorning_bindings::services::v1::V1Response;
use log::*;

use crate::config::{AccountConfig, APPLICATIONS};
use crate::error::Error as CError;
use crate::functions::{map_args, prompt_not_present};

const ARGS: &[&str] = &["path"];

pub fn cat(mut map: HashMap<String, String>, args: Vec<String>) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You must be logged in to view user files");
        return Err(CError::StrErr("not logged in").into());
    }

    prompt_not_present("Path", "path", &mut map);

    let prefix = PathBuf::from(map.get("prefix").unwrap_or(&String::new()));
    let path = prefix.join(map.get("path").unwrap());

    if !path.has_root() {
        error!("User file paths must start with root `/`");
        return Err(CError::StrErr("invalid file path").into());
    }

    let path = path.to_str().unwrap().to_string();

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap();

    let url = format!(
        "{}://{instance}/api/storage/v1/file/{token}/{}",
        if map.contains_key("http") {
            "http"
        } else {
            "https"
        },
        &path[1..]
    );

    info!("Sending request");
    let response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        error!(
            "Server responded with code `{}`",
            response.status().as_u16()
        );
        let s = response.text()?;
        let res: V1Response = serde_json::from_str(&s)?;

        match res {
            V1Response::Error { kind } => return Err(kind.into()),
            _ => unreachable!(),
        }
    }

    let cache_path = dirs::cache_dir()
        .unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .join("downloads")
        .join(map.get("id").unwrap())
        .join(&path[1..]);
    fs::create_dir_all(cache_path.parent().unwrap())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&cache_path)?;

    file.write_all(&response.bytes()?[..])?;

    APPLICATIONS.get_mut().open(&cache_path)?;

    Ok("Opened".to_string())
}
