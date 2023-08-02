use std::collections::HashMap;
use std::error::Error;

use goodmorning_bindings::services::v1::{V1Response, V1Unqueue};
use log::*;

use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present};

use crate::config::AccountConfig;

const ARGS: &[&str] = &["taskid"];

pub fn unqueue(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    prompt_not_present("Task ID", "taskid", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!("{}/api/jobs/v1/unqueue", instance);

    let taskid = map.get("taskid").unwrap().parse::<u64>()?;
    let token = map.get("token").unwrap().to_string();

    let body = V1Unqueue { token, id: taskid };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Failed to regenerate token");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Unqueued => {
            info!("Unqueued")
        }
        _ => unreachable!(),
    }

    Ok(String::from("Saved"))
}
