use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod accounts;
mod storage;
mod utils;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    let mut map = HashMap::new();
    map.extend(accounts::commands());
    map.extend(storage::commands());
    map.extend(utils::commands());

    map
}
