use std::collections::HashMap;

use crate::traits::types::CommandFnType;
use log::*;

pub enum Command {
    Command(CommandFnType),
    Category(HashMap<&'static str, Command>),
}

impl From<CommandFnType> for Command {
    fn from(value: CommandFnType) -> Self {
        Self::Command(value)
    }
}

impl From<HashMap<&'static str, Command>> for Command {
    fn from(value: HashMap<&'static str, Command>) -> Self {
        Self::Category(value)
    }
}

impl From<HashMap<&'static str, CommandFnType>> for Command {
    fn from(value: HashMap<&'static str, CommandFnType>) -> Self {
        let mut map = HashMap::new();

        value.into_iter().for_each(|(name, f)| {
            let _ = map.insert(name, Self::from(f));
        });

        map.into()
    }
}

impl Command {
    pub fn run(&self, map: HashMap<String, String>, args: Vec<String>) {
        match self {
            Self::Command(command) => match command(map, args) {
                Ok(msg) => debug!("Command finished with message `{msg}`"),
                Err(e) => error!("Command exited with error `{e}`"),
            },
            Self::Category(cog) if args.is_empty() || !cog.contains_key(args[0].as_str()) => {
                error!("No such command")
            }
            Self::Category(cog) => cog
                .get(args[0].as_str())
                .unwrap()
                .run(map, args.into_iter().skip(1).collect()),
        }
    }

    pub fn extend_map(self, map: &mut HashMap<&str, Self>) {
        match self {
            Self::Command(_) => panic!("wrong type buddy"),
            Self::Category(cog) => cog.into_iter().for_each(|(key, f)| {
                let _ = map.insert(key, f);
            }),
        }
    }
}
