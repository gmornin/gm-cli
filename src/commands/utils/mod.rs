use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod clean;
mod help;
mod version;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([
        ("clean", Box::new(clean::clean) as CommandFnType),
        ("help", Box::new(help::help) as CommandFnType),
        ("version", Box::new(version::version) as CommandFnType),
    ])
}
