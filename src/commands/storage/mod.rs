use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod cat;
mod ls;
mod upload;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([
        ("ls", Box::new(ls::ls) as CommandFnType),
        ("cat", Box::new(cat::cat) as CommandFnType),
        ("upload", Box::new(upload::upload) as CommandFnType),
    ])
}
