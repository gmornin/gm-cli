use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod cat;
mod cp;
mod ls;
mod mkdir;
mod rm;
mod touch;
mod upload;
mod vis;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([
        ("ls", Box::new(ls::ls) as CommandFnType),
        ("cat", Box::new(cat::cat) as CommandFnType),
        ("upload", Box::new(upload::upload) as CommandFnType),
        ("cp", Box::new(cp::cp) as CommandFnType),
        ("mkdir", Box::new(mkdir::mkdir) as CommandFnType),
        ("rm", Box::new(rm::rm) as CommandFnType),
        ("touch", Box::new(touch::touch) as CommandFnType),
        ("vis", Box::new(vis::vis) as CommandFnType),
    ])
}
