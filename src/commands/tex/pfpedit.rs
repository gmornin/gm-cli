use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::{V1Response, V1TokenOnly};
use log::*;
use reqwest::blocking::multipart::{Form, Part};

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present};

const ARGS: &[&str] = &["file"];

pub fn pfpedit(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;

    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    let instance = map.get("instance").unwrap().to_string();
    let token = map.get("token").unwrap().to_string();

    if map.contains_key("reset") {
        let url = format!("{}/api/generic/v1/reset-pfp", instance);
        let body = V1TokenOnly { token };

        let res: V1Response = post(&url, body, map.contains_key("http"))?;
        match res {
            V1Response::Error { kind } => {
                error!("Cannot reset profile");
                return Err(kind.into());
            }
            V1Response::PfpReset => {
                info!("Profile reset");
                return Ok("Ran".to_string());
            }
            _ => unreachable!(),
        }
    }

    prompt_not_present("Profile image path", "file", &mut map);

    let file = PathBuf::from(map.get("file").unwrap());

    if !file.exists() {
        error!("File to upload doesn't seem to exist");
        return Err(CError::StrErr("file not found").into());
    }

    let mut file = OpenOptions::new().read(true).open(&file)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let form = Form::new().part(
        "file",
        Part::bytes(buffer)
            .file_name("filename.ext")
            .mime_str("application/octet-stream")?,
    );

    let url = format!(
        "{}://{}/api/generic/v1/set-pfp/{token}",
        if map.contains_key("http") {
            "http"
        } else {
            "https"
        },
        instance
    );

    info!("Sending request and uploading file");
    let res = reqwest::blocking::Client::new()
        .post(url)
        .multipart(form)
        .send()?
        .text()?;

    let res: V1Response = match serde_json::from_str(&res) {
        Ok(res) => res,
        Err(e) => {
            error!("Cannot parse response");
            info!("{}", res);
            return Err(e.into());
        }
    };

    match res {
        V1Response::Error { kind } => {
            error!("Cannot load profile");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::ProfileUpdated => {
            info!("Profile updated");
        }
        _ => unreachable!(),
    }
    Ok("Ran".to_string())
}
