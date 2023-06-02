use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod accounts;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    let mut map = HashMap::new();
    map.extend(accounts::commands());

    map
}
