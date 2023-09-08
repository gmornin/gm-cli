use std::any::Any;
use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::{V1Job, V1Response, V1TokenOnly};
use goodmorning_bindings::structs::TexCompileDisplay;
use goodmorning_bindings::traits::SerdeAny;
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::post;

pub fn jobs(map: HashMap<String, String>, _args: Vec<String>) -> Result<String, Box<dyn Error>> {
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    let instance = map.get("instance").unwrap();
    let url = format!("{}/api/jobs/v1/jobs", instance);
    let token = map.get("token").unwrap().to_string();

    let body = V1TokenOnly { token };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("Failed to change name");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Jobs { current, queue } => {
            if current.is_empty() && queue.is_empty() {
                println!("You have no jobs");
                return Ok("No jobs".to_string());
            }
            if !queue.is_empty() {
                println!(
                    "Queued:\n{}",
                    queue
                        .iter()
                        .map(|job| format!("\t{}", job_display(job)))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            if !current.is_empty() {
                println!(
                    "Running jobs:\n{}",
                    current
                        .iter()
                        .map(|job| format!("\t{}", job_display(job)))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        }
        _ => unreachable!(),
    }

    Ok(String::from("Deleted"))
}

fn job_display(job: &V1Job) -> String {
    format!("{}: {}", job.id, task_display(&job.task))
    // todo!()
}

#[allow(clippy::borrowed_box)]
fn task_display(task: &Box<dyn SerdeAny>) -> String {
    let task_any: Box<dyn Any> = task.clone();
    if let Ok(TexCompileDisplay {
        from,
        to,
        compiler,
        path,
    }) = task_any.downcast::<TexCompileDisplay>().map(|v| *v)
    {
        return format!("compiling `{path}` from {from:?} to {to:?} with compiler {compiler:?}");
    }

    serde_yaml::to_string(task).unwrap()
}
