use std::{
    collections::HashMap,
    error::Error,
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::Path,
    process::Stdio,
};

use log::*;
use serde::{Deserialize, Serialize};
use singlyton::SingletonUninit;

use crate::traits::ConfigTriat;

pub static APPLICATIONS: SingletonUninit<ApplicationsConfig> = SingletonUninit::uninit();

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ApplicationsConfig(Vec<HashMap<String, String>>);

impl ConfigTriat for ApplicationsConfig {
    const NAME: &'static str = "applications";

    fn load() -> Result<Self, Box<dyn Error>> {
        let path = Self::path();

        debug!("Reading config file at {:?}", Self::path());

        let mut config = if path.exists() {
            let s = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => {
                    return Err(crate::error::Error::StringErr(format!(
                        "unable to read config file at `{path:?}`: {e}",
                    ))
                    .into())
                }
            };

            match serde_yaml::from_str(&s) {
                Ok(v) => v,
                Err(e) => {
                    return Err(crate::error::Error::StringErr(format!(
                        "error parsing config file at `{path:?}`: {e}"
                    ))
                    .into())
                }
            }
        } else {
            info!("No config file found at {:?}, using default", Self::path());
            Self::default()
        };

        debug!("Saving config file after load to {:?}", Self::path());

        config.sort();
        config.save()?;
        APPLICATIONS.init(config.clone());
        Ok(config)
    }
}

impl ApplicationsConfig {
    pub fn sort(&mut self) {
        self.0.sort_by(|this, other| {
            let this_str = this.keys().last().unwrap();
            let other_str = other.keys().last().unwrap();

            if this_str == other_str {
                warn!("There seemed to be a double entry on `{this_str}`");
                warn!(
                    "Chech your configuration at {:?} to remove it",
                    Self::path()
                );
            }

            let this_complexity = this_str.as_bytes().iter().filter(|b| b == &&b'.').count();
            let other_complexity = other_str.as_bytes().iter().filter(|b| b == &&b'.').count();

            if this_complexity != other_complexity {
                return other_complexity.cmp(&this_complexity);
            }

            other_str.len().cmp(&this_str.len())
        })
    }

    pub fn get(&self, k: &str) -> Option<&str> {
        Some(
            self.0
                .iter()
                .find(|map| map.keys().last().unwrap() == k)?
                .values()
                .last()
                .unwrap(),
        )
    }

    pub fn prompt_get(&mut self, k: &str) -> Result<String, Box<dyn Error>> {
        if let Some(c) = self.get(k) {
            return Ok(c.to_string());
        }

        let mut stdout = io::stdout();
        let stdin = io::stdin();

        println!("What command would you like to open files with a `{k}` extension?");
        println!(
            "Type the full command, use `{{path}}` as a placeholder, such as `firefox {{path}}`"
        );
        print!("> ");
        stdout.flush().unwrap();

        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let cmd = buf.trim();
        self.0
            .push(HashMap::from([(k.to_string(), cmd.to_string())]));
        self.save()?;
        Ok(cmd.to_string())
    }

    pub fn open(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let cmd = self
            .prompt_get(path.extension().unwrap_or(OsStr::new("")).to_str().unwrap())?
            .replace("{path}", path.to_str().unwrap());
        info!("Opening file with command `{cmd}`");
        let mut cmd = execute::command(cmd)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()?;

        let status = cmd.wait()?;
        if !status.success() {
            println!(
                "Command failed with exit code {}.",
                status.code().unwrap_or(-1)
            );
        }

        if matches!(cmd.try_wait(), Ok(Some(_))) {
            return Ok(());
        }

        cmd.kill()?;
        Ok(())
    }
}
