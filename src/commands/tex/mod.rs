use std::collections::HashMap;

use crate::traits::types::CommandFnType;

mod compile;
mod pfpedit;
mod profile;
mod publish;
mod publishes;
mod set_profile;

pub fn commands() -> HashMap<&'static str, CommandFnType> {
    HashMap::from([
        ("tprofile", Box::new(profile::profile) as CommandFnType),
        (
            "tpfedit",
            Box::new(set_profile::set_profile) as CommandFnType,
        ),
        ("tpfpedit", Box::new(pfpedit::pfpedit) as CommandFnType),
        ("compile", Box::new(compile::compile) as CommandFnType),
        ("publish", Box::new(publish::publish) as CommandFnType),
        ("publishes", Box::new(publishes::publishes) as CommandFnType),
    ])
}
