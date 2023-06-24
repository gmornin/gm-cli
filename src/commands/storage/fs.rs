use std::error::Error;
use std::{collections::HashMap, path::PathBuf};

use crate::error::Error as CError;
use crate::functions::{args_parse, prompt_cmd, resolve_path};
use crate::{
    config::AccountConfig,
    functions::{map_args, prompt_not_present},
};
use log::*;

use super::commands;

const ARGS: &[&str] = &["path"];

pub fn fs(mut map: HashMap<String, String>, args: Vec<String>) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You must be logged in to view user files");
        return Err(CError::StrErr("not logged in").into());
    }

    prompt_not_present("Path", "path", &mut map);

    let path = map.get("path").unwrap();
    let mut pathbuf = PathBuf::from(path);

    let mut commands = commands();
    let _ = commands.remove("fs").unwrap();

    loop {
        if !pathbuf.has_root() {
            no_root();
            info!("Defaulting to `/`");
            pathbuf = PathBuf::from("/");
        }
        let cmd = prompt_cmd();
        let mut map = map.clone();
        args_parse(&cmd, &mut map);

        match cmd
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => {}
            ["cd", path] => pathbuf = resolve_path(&pathbuf.join(path)),
            ["pwd"] => println!("{}", pathbuf.to_str().unwrap()),
            ["exit"] => break,
            [other, ..] if commands.contains_key(other) => {
                let mut map = map.clone();
                map.insert("prefix".to_string(), pathbuf.display().to_string());
                args_parse(&cmd[1..], &mut map);
                run_command(commands.get(other).unwrap()(map, cmd[1..].to_vec()));
            }
            _ => error!("Unknown command"),
        }
    }

    Ok(String::from("Exited"))
}

fn no_root() {
    error!("Invalid path, all paths must start with a root `/`");
}

fn run_command(res: Result<String, Box<dyn Error>>) {
    if let Err(e) = res {
        error!("Error: {e}")
    }
}
