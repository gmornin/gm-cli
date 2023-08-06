use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod compile;
mod pfpedit;
mod profile;
mod publish;
mod set_profile;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([
        ("profile", Box::new(profile::profile) as CommandFnType),
        (
            "pfedit",
            Box::new(set_profile::set_profile) as CommandFnType,
        ),
        ("pfpedit", Box::new(pfpedit::pfpedit) as CommandFnType),
        ("compile", Box::new(compile::compile) as CommandFnType),
        ("publish", Box::new(publish::publish) as CommandFnType),
    ])
}
