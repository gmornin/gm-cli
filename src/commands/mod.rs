use std::collections::HashMap;

use crate::Command;

mod accounts;
mod jobs;
mod storage;
pub mod tex;
mod utils;

pub fn commands() -> Command {
    let mut map: HashMap<&str, Command> = HashMap::new();
    // map.insert("tex", tex::commands().into());
    Command::from(tex::commands()).extend_map(&mut map);
    Command::from(accounts::commands()).extend_map(&mut map);
    Command::from(storage::commands()).extend_map(&mut map);
    Command::from(utils::commands()).extend_map(&mut map);
    Command::from(jobs::commands()).extend_map(&mut map);
    map.into()
}
