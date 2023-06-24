use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::{V1Error, V1Response};
use log::*;
use reqwest::blocking::multipart::{Form, Part};

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, prompt_not_present};

const ARGS: &[&str] = &["file", "path"];

pub fn upload(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You must be logged in to view user files");
        return Err(CError::StrErr("not logged in").into());
    }

    prompt_not_present("File path", "file", &mut map);
    prompt_not_present("Destination path", "path", &mut map);

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap();
    let file = PathBuf::from(map.get("file").unwrap());
    let prefix = PathBuf::from(map.get("prefix").unwrap_or(&String::new()));
    let path = prefix.join(map.get("path").unwrap());

    if !path.has_root() {
        error!("User file paths must start with root `/`");
        return Err(CError::StrErr("invalid file path").into());
    }

    let path = path.to_str().unwrap().to_string();

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

    let overwrite = map.contains_key("overwrite");

    let url = format!(
        "{}://{instance}/api/storage/v1/{}/{token}/{}",
        if map.contains_key("http") {
            "http"
        } else {
            "https"
        },
        if overwrite {
            "upload-overwrite"
        } else {
            "upload"
        },
        &path[1..]
    );

    info!("Sending request and uploading file");
    let res: V1Response = reqwest::blocking::Client::new()
        .post(url)
        .multipart(form)
        .send()?
        .json()?;

    match res {
        V1Response::FileItemCreated => {
            info!("File item successfully created at `{path}`");
        }
        V1Response::Error {
            kind: V1Error::FileNotFound,
        } if overwrite => {
            error!("File not found");
            info!("Perhaps this means the file path is not occupied, and you should not include the `--overwrite` flag");
            return Err(CError::StrErr("Upload failed").into());
        }
        V1Response::Error { kind } => {
            if let V1Error::FileTypeMismatch { expected, got } = kind {
                error!("Expected `{expected}`, got `{got}`");
                return Err(CError::StrErr("Upload failed").into());
            }
            return Err(kind.into());
        }
        _ => unreachable!(),
    }

    Ok("Uploaded".to_string())
}
