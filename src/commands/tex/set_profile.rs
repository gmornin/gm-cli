use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::{V1Response, V1ProfileOnly};
use goodmorning_bindings::structs::ProfileDetail;
use log::*;

use crate::error::Error as CError;
use crate::functions::{
    details_from_string, details_list, details_prompt, get, map_args, prompt,
    prompt_cmd, post, display_profile_only,
};

const ARGS: &[&str] = &["username"];

pub fn set_profile(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap();
    let id = map.get("id").unwrap().parse()?;

    let url = format!("{}/api/tex/generic/v1/profile-only/{id}", instance);

    let res: V1Response = get(&url, map.contains_key("http"))?;

    let mut profile = match res {
        V1Response::Error { kind } => {
            error!("Cannot load profile");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::ProfileOnly { profile } => profile,
        _ => unreachable!(),
    };

    info!("Profile recieved");

    loop {
        println!("{}\nRun `help` to see a list of commands\n\n", display_profile_only(&profile, id, instance));
        let cmd = prompt_cmd();

        match cmd
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => {}
            ["desc", ..] if !cmd.is_empty() => profile.description = cmd[1..].join(" ").to_string(),
            ["detail"] => {
                let res = (|| -> Result<ProfileDetail, Box<dyn Error>> {
                    println!("{}\n", details_list());
                    let index: usize = prompt("What do you want to add (1-7):").parse()?;

                    let value = prompt(&details_prompt(index)?);
                    details_from_string(index, value)
                })();

                match res {
                    Ok(detail) => profile.details.push(detail),
                    Err(e) => error!("Invalid details: {e}"),
                }
            }
            ["rm", index] => {
                let index: usize = match index.parse() {
                    Ok(index) => index,
                    Err(e) => {
                        error!("{e}");
                        continue;
                    },
                };
                if index == 0 || index > profile.details.len() {
                    error!("Invalid index");
                    continue;
                }

                profile.details.remove(index-1);
            }
            ["write"] => {
                let body = V1ProfileOnly {
                    profile: profile.clone(),
                    token: token.clone(),
                };
                let url = format!("{}/api/tex/generic/v1/set-profile", instance);
                
                let res: V1Response = match post(&url, body, map.contains_key("http")) {
                    Ok(res) => res,
                    Err(e) => {
                        error!("{e}");
                        continue;
                    }
                };

                match res {
                    V1Response::ProfileUpdated => info!("Profile updated"),
                    V1Response::Error { kind } => {
                        error!("Recieved error {kind}")
                    }
                    _ => unreachable!()
                }
            }
            ["help"] => println!("[Commands]\n1. write\n2. exit\n3. rm [index]: remove detail\n4. detail: add detail\n5. desc [replace]: replace description with string"),
            ["exit"] => break,
            _ => error!("Unknown command"),
        }
    }

    Ok("Ran".to_string())
}
