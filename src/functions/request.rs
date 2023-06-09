use std::{env, error::Error};

use log::*;
use reqwest::header::USER_AGENT;
use serde::{de::DeserializeOwned, Serialize};

pub fn post<R: DeserializeOwned, T: Serialize + Sized>(
    url: &str,
    body: T,
    http: bool,
) -> Result<R, Box<dyn Error>> {
    if http {
        warn!("This request is sent using the insecure http protocol");
    }

    info!("Sending request");
    let res = reqwest::blocking::Client::new()
        .post(format!("{}://{url}", if http { "http" } else { "https" }))
        .header(USER_AGENT, &format!("gm-cli {}", env!("CARGO_PKG_VERSION")))
        .json(&body)
        .send();
    let res = match res {
        Ok(res) => res,
        Err(e) => {
            error!("Error sending request to `{url}`");
            return Err(crate::error::Error::StringErr(e.to_string()).into());
        }
    };
    debug!("Response recieved, deserializing");
    match res.json() {
        Ok(out) => Ok(out),
        Err(e) => {
            error!("Deserialization failed");
            Err(crate::error::Error::StringErr(e.to_string()).into())
        }
    }
}

pub fn get<R: DeserializeOwned>(url: &str, http: bool) -> Result<R, Box<dyn Error>> {
    if http {
        warn!("This request is sent using the insecure http protocol");
    }

    info!("Sending request");
    let res = reqwest::blocking::Client::new()
        .get(format!("{}://{url}", if http { "http" } else { "https" }))
        .header(USER_AGENT, &format!("gm-cli {}", env!("CARGO_PKG_VERSION")))
        .send();
    let res = match res {
        Ok(res) => res,
        Err(e) => {
            error!("Error sending request to `{url}`");
            return Err(crate::error::Error::StringErr(e.to_string()).into());
        }
    };
    debug!("Response recieved, deserializing");
    match res.json() {
        Ok(out) => Ok(out),
        Err(e) => {
            error!("Deserialization failed");
            Err(crate::error::Error::StringErr(e.to_string()).into())
        }
    }
}
