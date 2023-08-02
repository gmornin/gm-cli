use std::{collections::HashMap, error::Error};

use goodmorning_bindings::services::v1::{V1ProfileOnly, V1Response, V1TokenOnly};
use goodmorning_bindings::structs::{ContactDetail, ProfileCustomisable, ProfileDetail};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{
    contacts_from_string, contacts_list, contacts_prompt, details_from_string, details_list,
    details_prompt, display_profile_only, get, post, prompt, prompt_cmd, yes_msg,
};

pub fn set_profile(
    map: HashMap<String, String>,
    _args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    // map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap();
    let id = map.get("id").unwrap().parse()?;

    if map.contains_key("reset") {
        yes_msg("Are you sure you want to reset your profile?", &map);
        let url = format!("{}/api/tex/generic/v1/reset-profile", instance);
        let body = V1TokenOnly {
            token: token.to_string(),
        };

        let res: V1Response = post(&url, body, map.contains_key("http"))?;

        match res {
            V1Response::Error { kind } => {
                error!("Cannot load profile");
                return Err(CError::StringErr(kind.to_string()).into());
            }
            V1Response::ProfileUpdated => {
                info!("Profile reset");
            }
            _ => unreachable!(),
        }
        return Ok("Ran".to_string());
    }

    let url = format!("{}/api/tex/generic/v1/profile-only/id/{id}", instance);

    let res: V1Response = get(&url, map.contains_key("http"))?;

    let mut profile = match res {
        V1Response::Error { kind } => {
            error!("Cannot load profile");
            info!("Try running again with `--reset` flag");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::ProfileOnly { profile } => profile,
        _ => unreachable!(),
    };

    info!("Profile recieved");

    loop {
        let pferror = profile_error(&profile);

        if let Some(profile_error) = &pferror {
            warn!("{}", profile_error);
        }

        info!(
            "{}\nRun `help` to see a list of commands\n\n",
            display_profile_only(&profile, id, instance)
        );
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
                    let index: usize = prompt("What do you want to add (1-7)").parse()?;

                    let value = prompt(&details_prompt(index)?);
                    details_from_string(index, value)
                })();

                match res {
                    Ok(detail) => profile.details.push(detail),
                    Err(e) => error!("Invalid details: {e}"),
                }
            }
            ["contact"] => {
                let res = (|| -> Result<ContactDetail, Box<dyn Error>> {
                    println!("{}\n", contacts_list());
                    let index: usize = prompt("What do you want to add (1-13)").parse()?;

                    let value = prompt(&contacts_prompt(index)?);
                    contacts_from_string(index, value)
                })();
                match res {
                    Ok(detail) => profile.details.push(ProfileDetail::Contact { value: detail }),
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
                if let Some(profile_error) = pferror {
                    error!("{}", profile_error);
                    error!("Profile not updated");
                    continue;
                }

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
            ["help"] => println!("[Commands]\n1. write\n2. exit\n3. rm [index]: remove detail\n4. detail: add detail\n5. desc [replace]: replace description with string\n6. contact: add contact\n7. help\n"),
            ["exit"] => break,
            _ => error!("Unknown command"),
        }
    }

    Ok("Ran".to_string())
}

fn profile_error(profile: &ProfileCustomisable) -> Option<String> {
    if profile.description.len() > 2000 {
        Some("Description is too long (2000 max)".to_string())
    } else if profile.details.len() > 20 {
        Some("Details are too many (20 max)".to_string())
    } else if profile
        .details
        .iter()
        .filter(|detail| {
            matches!(
                detail,
                ProfileDetail::CakeDay { .. } | ProfileDetail::BirthDay { .. }
            )
        })
        .count()
        > 1
    {
        Some("Only one birthday or cake day allowed".to_string())
    } else if let Err(Some(i)) = profile.validate() {
        Some(format!(
            "Invalid detail {} = \"{:?}\", try running command `rm {}` to remove it",
            i + 1,
            profile.details[i],
            i + 1
        ))
    } else {
        None
    }
}
